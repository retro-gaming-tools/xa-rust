#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_02110c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2110c24); }
 let mut pc: u32 = 0x2110c24;
 loop { match pc {
 0x2110c24 => {
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x11u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.ecx))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x11u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.call(0x2110c34); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0211158c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x211158c); }
 let mut pc: u32 = 0x211158c;
 loop { match pc {
 0x211158c => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x21115a0; continue; }
  pc = 0x211159d; continue;
 }
 0x211159d => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x21115a0; continue;
 }
 0x21115a0 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { pc = 0x21115a8; continue; }
  pc = 0x21115a6; continue;
 }
 0x21115a6 => {
  { cpu.stos(2, false); }
  pc = 0x21115a8; continue;
 }
 0x21115a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_021115a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x21115a9); }
 let mut pc: u32 = 0x21115a9;
 loop { match pc {
 0x21115a9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x21115b0; continue;
 }
 0x21115b0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x21115b2; continue;
 }
 0x21115b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21115b2; continue; }
  pc = 0x21115be; continue;
 }
 0x21115be => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21115b0; continue; }
  pc = 0x21115c3; continue;
 }
 0x21115c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_021117df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x21117df); }
 let mut pc: u32 = 0x21117df;
 loop { match pc {
 0x21117df => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x2111817; continue; }
  pc = 0x21117f5; continue;
 }
 0x21117f5 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x2111807; continue; }
  pc = 0x21117f9; continue;
 }
 0x21117f9 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21117f9; continue; }
  pc = 0x2111804; continue;
 }
 0x2111804 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x2111807 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2111807; continue; }
  pc = 0x2111814; continue;
 }
 0x2111814 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x2111817 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x211182e; continue; }
  pc = 0x211181b; continue;
 }
 0x211181b => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x211181c; continue;
 }
 0x211181c => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x211181c; continue; }
  pc = 0x211182b; continue;
 }
 0x211182b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x211182e => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x211182e; continue; }
  pc = 0x211183b; continue;
 }
 0x211183b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0211183e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x211183e); }
 let mut pc: u32 = 0x211183e;
 loop { match pc {
 0x211183e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x22f1eb0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x22f1eacu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x2111a55; continue;
 }
 0x2111869 => {
  { let v: u32 = cpu.r32(0x22f1eacu32); cpu.ecx = v; }
  pc = 0x2111938; continue;
 }
 0x2111874 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x211187e; continue; }
  pc = 0x211187b; continue;
 }
 0x211187b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x211187e; continue;
 }
 0x211187e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x211188a; continue; }
  pc = 0x2111886; continue;
 }
 0x2111886 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x211188a; continue;
 }
 0x211188a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111896; continue; }
  pc = 0x2111892; continue;
 }
 0x2111892 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111896; continue;
 }
 0x2111896 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118a2; continue; }
  pc = 0x211189e; continue;
 }
 0x211189e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x21118a2; continue;
 }
 0x21118a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118ae; continue; }
  pc = 0x21118aa; continue;
 }
 0x21118aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x21118ae; continue;
 }
 0x21118ae => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118ba; continue; }
  pc = 0x21118b6; continue;
 }
 0x21118b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x21118ba; continue;
 }
 0x21118ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118c6; continue; }
  pc = 0x21118c2; continue;
 }
 0x21118c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x21118c6; continue;
 }
 0x21118c6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118d2; continue; }
  pc = 0x21118ce; continue;
 }
 0x21118ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x21118d2; continue;
 }
 0x21118d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118de; continue; }
  pc = 0x21118da; continue;
 }
 0x21118da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x21118de; continue;
 }
 0x21118de => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118ea; continue; }
  pc = 0x21118e6; continue;
 }
 0x21118e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x21118ea; continue;
 }
 0x21118ea => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21118f6; continue; }
  pc = 0x21118f2; continue;
 }
 0x21118f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x21118f6; continue;
 }
 0x21118f6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111902; continue; }
  pc = 0x21118fe; continue;
 }
 0x21118fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111902; continue;
 }
 0x2111902 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x211190e; continue; }
  pc = 0x211190a; continue;
 }
 0x211190a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x211190e; continue;
 }
 0x211190e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x211191a; continue; }
  pc = 0x2111916; continue;
 }
 0x2111916 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x211191a; continue;
 }
 0x211191a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111926; continue; }
  pc = 0x2111922; continue;
 }
 0x2111922 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x2111926; continue;
 }
 0x2111926 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111932; continue; }
  pc = 0x211192e; continue;
 }
 0x211192e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x2111932; continue;
 }
 0x2111932 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x2111938; continue;
 }
 0x2111938 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2111874; continue; }
  pc = 0x2111941; continue;
 }
 0x2111941 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x211194d; continue;
 }
 0x211194d => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111956; continue; }
  pc = 0x2111953; continue;
 }
 0x2111953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111956; continue;
 }
 0x2111956 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111962; continue; }
  pc = 0x211195e; continue;
 }
 0x211195e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111962; continue;
 }
 0x2111962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x211196e; continue;
 }
 0x211196e => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111977; continue; }
  pc = 0x2111974; continue;
 }
 0x2111974 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111977; continue;
 }
 0x2111977 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111983; continue; }
  pc = 0x211197f; continue;
 }
 0x211197f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111983; continue;
 }
 0x2111983 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x211198f; continue;
 }
 0x211198f => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111998; continue; }
  pc = 0x2111995; continue;
 }
 0x2111995 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111998; continue;
 }
 0x2111998 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119a4; continue; }
  pc = 0x21119a0; continue;
 }
 0x21119a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119a4; continue;
 }
 0x21119a4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x21119b0; continue;
 }
 0x21119b0 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119b9; continue; }
  pc = 0x21119b6; continue;
 }
 0x21119b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119b9; continue;
 }
 0x21119b9 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119c5; continue; }
  pc = 0x21119c1; continue;
 }
 0x21119c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119c5; continue;
 }
 0x21119c5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x21119cd; continue;
 }
 0x21119cd => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119d6; continue; }
  pc = 0x21119d3; continue;
 }
 0x21119d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119d6; continue;
 }
 0x21119d6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119e2; continue; }
  pc = 0x21119de; continue;
 }
 0x21119de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119e2; continue;
 }
 0x21119e2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x21119ea; continue;
 }
 0x21119ea => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119f3; continue; }
  pc = 0x21119f0; continue;
 }
 0x21119f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119f3; continue;
 }
 0x21119f3 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x21119ff; continue; }
  pc = 0x21119fb; continue;
 }
 0x21119fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x21119ff; continue;
 }
 0x21119ff => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x2111a07; continue;
 }
 0x2111a07 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111a10; continue; }
  pc = 0x2111a0d; continue;
 }
 0x2111a0d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111a10; continue;
 }
 0x2111a10 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111a1c; continue; }
  pc = 0x2111a18; continue;
 }
 0x2111a18 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111a1c; continue;
 }
 0x2111a1c => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x2111a3c; continue; }
  pc = 0x2111a24; continue;
 }
 0x2111a24 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111a2d; continue; }
  pc = 0x2111a2a; continue;
 }
 0x2111a2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111a2d; continue;
 }
 0x2111a2d => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111a39; continue; }
  pc = 0x2111a35; continue;
 }
 0x2111a35 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111a39; continue;
 }
 0x2111a39 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2111a3c; continue;
 }
 0x2111a3c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x2111a51; continue; }
  pc = 0x2111a44; continue;
 }
 0x2111a44 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x2111a4e; continue; }
  pc = 0x2111a4b; continue;
 }
 0x2111a4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x2111a4e; continue;
 }
 0x2111a4e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2111a51; continue;
 }
 0x2111a51 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x2111a55; continue;
 }
 0x2111a55 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x22f1eb0u32)); cpu.w32(0x22f1eb0u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x2111869; continue; }
  pc = 0x2111a61; continue;
 }
 0x2111a61 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02111a65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2111a65); }
 let mut pc: u32 = 0x2111a65;
 loop { match pc {
 0x2111a65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x22f1eb8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22f1eb4u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x22f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x2111a84; continue;
 }
 0x2111a84 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.sf { pc = 0x2111a93; continue; }
  pc = 0x2111a88; continue;
 }
 0x2111a88 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x22f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  pc = 0x2111a93; continue;
 }
 0x2111a93 => {
  { let v: u32 = cpu.r32(0x22f1eb8u32); cpu.edi = v; }
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x22f1eb4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x2111a84; continue; }
  pc = 0x2111adf; continue;
 }
 0x2111adf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02111ae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2111ae1); }
 let mut pc: u32 = 0x2111ae1;
 loop { match pc {
 0x2111ae1 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2111ae1; continue; }
  pc = 0x2111aeb; continue;
 }
 0x2111aeb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02120000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2120000); } r_02120000(cpu, 0x2120000); }
pub fn f_0212020b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x212020b); } r_02120000(cpu, 0x212020b); }
fn r_02120000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x2120000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2251a4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2251a48u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212018a; continue; }
  pc = 0x2120023; continue;
 }
 0x2120023 => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212006d; continue; }
  pc = 0x212002f; continue;
 }
 0x212002f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120064; continue; }
  pc = 0x2120053; continue;
 }
 0x2120053 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120064; continue;
 }
 0x2120064 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212006d; continue;
 }
 0x212006d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x21200a2; continue;
 }
 0x2120072 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21200a2; continue;
 }
 0x21200a2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ecx = v; }
  pc = 0x21200ab; continue;
 }
 0x21200ab => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21200ab; continue; }
  pc = 0x21200ce; continue;
 }
 0x21200ce => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x2120072; continue; }
  pc = 0x21200d8; continue;
 }
 0x21200d8 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x2120114; continue;
 }
 0x2120114 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2120114; continue; }
  pc = 0x2120137; continue;
 }
 0x2120137 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120185; continue; }
  pc = 0x2120147; continue;
 }
 0x2120147 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212017c; continue; }
  pc = 0x212016b; continue;
 }
 0x212016b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212017c; continue;
 }
 0x212017c => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120185; continue;
 }
 0x2120185 => {
  pc = 0x212027a; continue;
 }
 0x212018a => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21201d4; continue; }
  pc = 0x2120196; continue;
 }
 0x2120196 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21201cb; continue; }
  pc = 0x21201ba; continue;
 }
 0x21201ba => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21201cb; continue;
 }
 0x21201cb => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21201d4; continue;
 }
 0x21201d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x21201f7; continue;
 }
 0x21201d9 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { cpu.interrupt(0x10); }
  pc = 0x21201f7; continue;
 }
 0x21201ea => {
  { cpu.pushad(); }
  { let v: u8 = cpu.sbb8(((cpu.eax >> 8) as u8), cpu.r8(0x15896602u32)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xcd02251au32; cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0xd8b0c7du32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); cpu.w8(0xd8b0c7du32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.sbb8(((cpu.eax >> 8) as u8), cpu.r8(0x20c68302u32)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2120200; continue; }
  pc = 0x2120223; continue;
 }
 0x21201f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ecx = v; }
  pc = 0x2120200; continue;
 }
 0x2120200 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2120200; continue; }
  pc = 0x2120223; continue;
 }
 0x212020b => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x21201ea; continue; }
  pc = 0x212020d; continue;
 }
 0x212020d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x2120213); cpu.call(0xdffa61f0); if cpu.returned_to != 0x2120213 { return; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.call(0x2120214); return;
 }
 0x2120223 => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x21201d9; continue; }
  pc = 0x212022d; continue;
 }
 0x212022d => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x2120257; continue;
 }
 0x2120257 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2120257; continue; }
  pc = 0x212027a; continue;
 }
 0x212027a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212027c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212027c); }
 let mut pc: u32 = 0x212027c;
 loop { match pc {
 0x212027c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21202f1; continue; }
  pc = 0x21202b3; continue;
 }
 0x21202b3 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21202e8; continue; }
  pc = 0x21202d7; continue;
 }
 0x21202d7 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21202e8; continue;
 }
 0x21202e8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21202f1; continue;
 }
 0x21202f1 => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x212032f; continue;
 }
 0x212032f => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x2120350; continue; }
  pc = 0x2120337; continue;
 }
 0x2120337 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212032f; continue; }
  pc = 0x212034b; continue;
 }
 0x212034b => {
  pc = 0x2120474; continue;
 }
 0x2120350 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x21203fb; continue; }
  pc = 0x2120361; continue;
 }
 0x2120361 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21203c6; continue; }
  pc = 0x2120388; continue;
 }
 0x2120388 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21203bd; continue; }
  pc = 0x21203ac; continue;
 }
 0x21203ac => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21203bd; continue;
 }
 0x21203bd => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21203c6; continue;
 }
 0x21203c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212032f; continue; }
  pc = 0x21203f9; continue;
 }
 0x21203f9 => {
  pc = 0x2120474; continue;
 }
 0x21203fb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212045d; continue; }
  pc = 0x212041f; continue;
 }
 0x212041f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120454; continue; }
  pc = 0x2120443; continue;
 }
 0x2120443 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120454; continue;
 }
 0x2120454 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212045d; continue;
 }
 0x212045d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212032f; continue; }
  pc = 0x2120474; continue;
 }
 0x2120474 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02120477(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2120477); }
 let mut pc: u32 = 0x2120477;
 loop { match pc {
 0x2120477 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21204ec; continue; }
  pc = 0x21204ae; continue;
 }
 0x21204ae => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21204e3; continue; }
  pc = 0x21204d2; continue;
 }
 0x21204d2 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21204e3; continue;
 }
 0x21204e3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21204ec; continue;
 }
 0x21204ec => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x212052a; continue;
 }
 0x212052a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x212054b; continue; }
  pc = 0x2120532; continue;
 }
 0x2120532 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212052a; continue; }
  pc = 0x2120546; continue;
 }
 0x2120546 => {
  pc = 0x2120671; continue;
 }
 0x212054b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x21205f8; continue; }
  pc = 0x212055c; continue;
 }
 0x212055c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21205c1; continue; }
  pc = 0x2120583; continue;
 }
 0x2120583 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21205b8; continue; }
  pc = 0x21205a7; continue;
 }
 0x21205a7 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21205b8; continue;
 }
 0x21205b8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21205c1; continue;
 }
 0x21205c1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212052a; continue; }
  pc = 0x21205f6; continue;
 }
 0x21205f6 => {
  pc = 0x2120671; continue;
 }
 0x21205f8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212065a; continue; }
  pc = 0x212061c; continue;
 }
 0x212061c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120651; continue; }
  pc = 0x2120640; continue;
 }
 0x2120640 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120651; continue;
 }
 0x2120651 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212065a; continue;
 }
 0x212065a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212052a; continue; }
  pc = 0x2120671; continue;
 }
 0x2120671 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02120674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2120674); }
 let mut pc: u32 = 0x2120674;
 loop { match pc {
 0x2120674 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21206c4; continue; }
  pc = 0x2120686; continue;
 }
 0x2120686 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21206bb; continue; }
  pc = 0x21206aa; continue;
 }
 0x21206aa => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21206bb; continue;
 }
 0x21206bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21206c4; continue;
 }
 0x21206c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x2300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x21206e6; continue; }
  pc = 0x21206db; continue;
 }
 0x21206db => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x21206e6; continue;
 }
 0x21206e6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21206fd; continue;
 }
 0x21206ed => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21206fd; continue;
 }
 0x21206fd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21206ed; continue; }
  pc = 0x2120700; continue;
 }
 0x2120700 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21206e6; continue; }
  pc = 0x2120709; continue;
 }
 0x2120709 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120720; continue;
 }
 0x2120710 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120720; continue;
 }
 0x2120720 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120710; continue; }
  pc = 0x2120723; continue;
 }
 0x2120723 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120791; continue; }
  pc = 0x2120753; continue;
 }
 0x2120753 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120788; continue; }
  pc = 0x2120777; continue;
 }
 0x2120777 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120788; continue;
 }
 0x2120788 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120791; continue;
 }
 0x2120791 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x21207af; continue;
 }
 0x21207af => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21207c6; continue;
 }
 0x21207b6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21207c6; continue;
 }
 0x21207c6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21207b6; continue; }
  pc = 0x21207c9; continue;
 }
 0x21207c9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21207af; continue; }
  pc = 0x21207d2; continue;
 }
 0x21207d2 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x21207e9; continue;
 }
 0x21207d9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21207e9; continue;
 }
 0x21207e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21207d9; continue; }
  pc = 0x21207ec; continue;
 }
 0x21207ec => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120841; continue; }
  pc = 0x2120803; continue;
 }
 0x2120803 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120838; continue; }
  pc = 0x2120827; continue;
 }
 0x2120827 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120838; continue;
 }
 0x2120838 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120841; continue;
 }
 0x2120841 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x212085e; continue;
 }
 0x212084e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212085e; continue;
 }
 0x212085e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212084e; continue; }
  pc = 0x2120861; continue;
 }
 0x2120861 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x212087f; continue;
 }
 0x212087f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120896; continue;
 }
 0x2120886 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120896; continue;
 }
 0x2120896 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120886; continue; }
  pc = 0x2120899; continue;
 }
 0x2120899 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212087f; continue; }
  pc = 0x21208a2; continue;
 }
 0x21208a2 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21208b9; continue;
 }
 0x21208a9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21208b9; continue;
 }
 0x21208b9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21208a9; continue; }
  pc = 0x21208bc; continue;
 }
 0x21208bc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212092a; continue; }
  pc = 0x21208ec; continue;
 }
 0x21208ec => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120921; continue; }
  pc = 0x2120910; continue;
 }
 0x2120910 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120921; continue;
 }
 0x2120921 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212092a; continue;
 }
 0x212092a => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2120948; continue;
 }
 0x2120948 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x212095f; continue;
 }
 0x212094f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212095f; continue;
 }
 0x212095f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212094f; continue; }
  pc = 0x2120962; continue;
 }
 0x2120962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120948; continue; }
  pc = 0x212096b; continue;
 }
 0x212096b => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x2120982; continue;
 }
 0x2120972 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120982; continue;
 }
 0x2120982 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120972; continue; }
  pc = 0x2120985; continue;
 }
 0x2120985 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21209da; continue; }
  pc = 0x212099c; continue;
 }
 0x212099c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21209d1; continue; }
  pc = 0x21209c0; continue;
 }
 0x21209c0 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21209d1; continue;
 }
 0x21209d1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21209da; continue;
 }
 0x21209da => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x21209f7; continue;
 }
 0x21209e7 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21209f7; continue;
 }
 0x21209f7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21209e7; continue; }
  pc = 0x21209fa; continue;
 }
 0x21209fa => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2120a18; continue;
 }
 0x2120a18 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120a2f; continue;
 }
 0x2120a1f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120a2f; continue;
 }
 0x2120a2f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120a1f; continue; }
  pc = 0x2120a32; continue;
 }
 0x2120a32 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120a18; continue; }
  pc = 0x2120a3b; continue;
 }
 0x2120a3b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120a90; continue; }
  pc = 0x2120a52; continue;
 }
 0x2120a52 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120a87; continue; }
  pc = 0x2120a76; continue;
 }
 0x2120a76 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120a87; continue;
 }
 0x2120a87 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120a90; continue;
 }
 0x2120a90 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2120a9b; continue;
 }
 0x2120a9b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120ab2; continue;
 }
 0x2120aa2 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120ab2; continue;
 }
 0x2120ab2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120aa2; continue; }
  pc = 0x2120ab5; continue;
 }
 0x2120ab5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120a9b; continue; }
  pc = 0x2120abe; continue;
 }
 0x2120abe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120ad5; continue;
 }
 0x2120ac5 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120ad5; continue;
 }
 0x2120ad5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120ac5; continue; }
  pc = 0x2120ad8; continue;
 }
 0x2120ad8 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120b46; continue; }
  pc = 0x2120b08; continue;
 }
 0x2120b08 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120b3d; continue; }
  pc = 0x2120b2c; continue;
 }
 0x2120b2c => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120b3d; continue;
 }
 0x2120b3d => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120b46; continue;
 }
 0x2120b46 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2120b64; continue;
 }
 0x2120b64 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120b7b; continue;
 }
 0x2120b6b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120b7b; continue;
 }
 0x2120b7b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120b6b; continue; }
  pc = 0x2120b7e; continue;
 }
 0x2120b7e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120b64; continue; }
  pc = 0x2120b87; continue;
 }
 0x2120b87 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x2120b9e; continue;
 }
 0x2120b8e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120b9e; continue;
 }
 0x2120b9e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120b8e; continue; }
  pc = 0x2120ba1; continue;
 }
 0x2120ba1 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120bf6; continue; }
  pc = 0x2120bb8; continue;
 }
 0x2120bb8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120bed; continue; }
  pc = 0x2120bdc; continue;
 }
 0x2120bdc => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120bed; continue;
 }
 0x2120bed => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120bf6; continue;
 }
 0x2120bf6 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x2120c13; continue;
 }
 0x2120c03 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120c13; continue;
 }
 0x2120c13 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120c03; continue; }
  pc = 0x2120c16; continue;
 }
 0x2120c16 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2120d32; continue; }
  pc = 0x2120c3f; continue;
 }
 0x2120c3f => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x2120c44; continue;
 }
 0x2120c44 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120c5b; continue;
 }
 0x2120c4b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120c5b; continue;
 }
 0x2120c5b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120c4b; continue; }
  pc = 0x2120c5e; continue;
 }
 0x2120c5e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120c44; continue; }
  pc = 0x2120c67; continue;
 }
 0x2120c67 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120c7e; continue;
 }
 0x2120c6e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120c7e; continue;
 }
 0x2120c7e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120c6e; continue; }
  pc = 0x2120c81; continue;
 }
 0x2120c81 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120cef; continue; }
  pc = 0x2120cb1; continue;
 }
 0x2120cb1 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120ce6; continue; }
  pc = 0x2120cd5; continue;
 }
 0x2120cd5 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120ce6; continue;
 }
 0x2120ce6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120cef; continue;
 }
 0x2120cef => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x2120d0d; continue;
 }
 0x2120d0d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120d24; continue;
 }
 0x2120d14 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120d24; continue;
 }
 0x2120d24 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120d14; continue; }
  pc = 0x2120d27; continue;
 }
 0x2120d27 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120d0d; continue; }
  pc = 0x2120d30; continue;
 }
 0x2120d30 => {
  pc = 0x2120d5a; continue;
 }
 0x2120d32 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x2120d37; continue;
 }
 0x2120d37 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2120d4e; continue;
 }
 0x2120d3e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2120d4e; continue;
 }
 0x2120d4e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120d3e; continue; }
  pc = 0x2120d51; continue;
 }
 0x2120d51 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120d37; continue; }
  pc = 0x2120d5a; continue;
 }
 0x2120d5a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02120d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2120d5c); }
 let mut pc: u32 = 0x2120d5c;
 loop { match pc {
 0x2120d5c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2120d7b; continue; }
  pc = 0x2120d78; continue;
 }
 0x2120d78 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x2120d7b; continue;
 }
 0x2120d7b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120de5; continue; }
  pc = 0x2120da7; continue;
 }
 0x2120da7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120ddc; continue; }
  pc = 0x2120dcb; continue;
 }
 0x2120dcb => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120ddc; continue;
 }
 0x2120ddc => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120de5; continue;
 }
 0x2120de5 => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x2120e20; continue;
 }
 0x2120e20 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x2120e47; continue; }
  pc = 0x2120e28; continue;
 }
 0x2120e28 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2120e36; continue;
 }
 0x2120e2d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2120e36; continue;
 }
 0x2120e36 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e2d; continue; }
  pc = 0x2120e39; continue;
 }
 0x2120e39 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e20; continue; }
  pc = 0x2120e42; continue;
 }
 0x2120e42 => {
  pc = 0x212110c; continue;
 }
 0x2120e47 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x2120f13; continue; }
  pc = 0x2120e58; continue;
 }
 0x2120e58 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2120e66; continue;
 }
 0x2120e5d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2120e66; continue;
 }
 0x2120e66 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e5d; continue; }
  pc = 0x2120e69; continue;
 }
 0x2120e69 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120ec0; continue; }
  pc = 0x2120e82; continue;
 }
 0x2120e82 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120eb7; continue; }
  pc = 0x2120ea6; continue;
 }
 0x2120ea6 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120eb7; continue;
 }
 0x2120eb7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120ec0; continue;
 }
 0x2120ec0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2120ee0; continue;
 }
 0x2120ed7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2120ee0; continue;
 }
 0x2120ee0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120ed7; continue; }
  pc = 0x2120ee3; continue;
 }
 0x2120ee3 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e20; continue; }
  pc = 0x2120f0e; continue;
 }
 0x2120f0e => {
  pc = 0x212110c; continue;
 }
 0x2120f13 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x212106f; continue; }
  pc = 0x2120f21; continue;
 }
 0x2120f21 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x2120feb; continue; }
  pc = 0x2120f2a; continue;
 }
 0x2120f2a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2120f38; continue;
 }
 0x2120f2f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2120f38; continue;
 }
 0x2120f38 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120f2f; continue; }
  pc = 0x2120f3b; continue;
 }
 0x2120f3b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2120fb2; continue; }
  pc = 0x2120f74; continue;
 }
 0x2120f74 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2120fa9; continue; }
  pc = 0x2120f98; continue;
 }
 0x2120f98 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2120fa9; continue;
 }
 0x2120fa9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2120fb2; continue;
 }
 0x2120fb2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e20; continue; }
  pc = 0x2120fe6; continue;
 }
 0x2120fe6 => {
  pc = 0x212110c; continue;
 }
 0x2120feb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2120ff9; continue;
 }
 0x2120ff0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2120ff9; continue;
 }
 0x2120ff9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120ff0; continue; }
  pc = 0x2120ffc; continue;
 }
 0x2120ffc => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121053; continue; }
  pc = 0x2121015; continue;
 }
 0x2121015 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212104a; continue; }
  pc = 0x2121039; continue;
 }
 0x2121039 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212104a; continue;
 }
 0x212104a => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121053; continue;
 }
 0x2121053 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e20; continue; }
  pc = 0x212106a; continue;
 }
 0x212106a => {
  pc = 0x212110c; continue;
 }
 0x212106f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x212107d; continue;
 }
 0x2121074 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x212107d; continue;
 }
 0x212107d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121074; continue; }
  pc = 0x2121080; continue;
 }
 0x2121080 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21210dd; continue; }
  pc = 0x212109f; continue;
 }
 0x212109f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21210d4; continue; }
  pc = 0x21210c3; continue;
 }
 0x21210c3 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21210d4; continue;
 }
 0x21210d4 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21210dd; continue;
 }
 0x21210dd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2120e20; continue; }
  pc = 0x212110c; continue;
 }
 0x212110c => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212110f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212110f); }
 let mut pc: u32 = 0x212110f;
 loop { match pc {
 0x212110f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x212112e; continue; }
  pc = 0x212112b; continue;
 }
 0x212112b => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x212112e; continue;
 }
 0x212112e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121198; continue; }
  pc = 0x212115a; continue;
 }
 0x212115a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212118f; continue; }
  pc = 0x212117e; continue;
 }
 0x212117e => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212118f; continue;
 }
 0x212118f => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121198; continue;
 }
 0x2121198 => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x21211d3; continue;
 }
 0x21211d3 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x21211f4; continue; }
  pc = 0x21211db; continue;
 }
 0x21211db => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21211e3; continue;
 }
 0x21211e0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21211e3; continue;
 }
 0x21211e3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211e0; continue; }
  pc = 0x21211e6; continue;
 }
 0x21211e6 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211d3; continue; }
  pc = 0x21211ef; continue;
 }
 0x21211ef => {
  pc = 0x212148b; continue;
 }
 0x21211f4 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x21212b0; continue; }
  pc = 0x2121205; continue;
 }
 0x2121205 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x212120d; continue;
 }
 0x212120a => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x212120d; continue;
 }
 0x212120d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212120a; continue; }
  pc = 0x2121210; continue;
 }
 0x2121210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121267; continue; }
  pc = 0x2121229; continue;
 }
 0x2121229 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212125e; continue; }
  pc = 0x212124d; continue;
 }
 0x212124d => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212125e; continue;
 }
 0x212125e => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121267; continue;
 }
 0x2121267 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2121281; continue;
 }
 0x212127e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2121281; continue;
 }
 0x2121281 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212127e; continue; }
  pc = 0x2121284; continue;
 }
 0x2121284 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121295; continue;
 }
 0x2121292 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2121295; continue;
 }
 0x2121295 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121292; continue; }
  pc = 0x2121298; continue;
 }
 0x2121298 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211d3; continue; }
  pc = 0x21212ab; continue;
 }
 0x21212ab => {
  pc = 0x212148b; continue;
 }
 0x21212b0 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x21213f8; continue; }
  pc = 0x21212be; continue;
 }
 0x21212be => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x212137a; continue; }
  pc = 0x21212c7; continue;
 }
 0x21212c7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21212cf; continue;
 }
 0x21212cc => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21212cf; continue;
 }
 0x21212cf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21212cc; continue; }
  pc = 0x21212d2; continue;
 }
 0x21212d2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x21212eb; continue;
 }
 0x21212e8 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21212eb; continue;
 }
 0x21212eb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21212e8; continue; }
  pc = 0x21212ee; continue;
 }
 0x21212ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121345; continue; }
  pc = 0x2121307; continue;
 }
 0x2121307 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212133c; continue; }
  pc = 0x212132b; continue;
 }
 0x212132b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212133c; continue;
 }
 0x212133c => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121345; continue;
 }
 0x2121345 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x212135f; continue;
 }
 0x212135c => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x212135f; continue;
 }
 0x212135f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212135c; continue; }
  pc = 0x2121362; continue;
 }
 0x2121362 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211d3; continue; }
  pc = 0x2121375; continue;
 }
 0x2121375 => {
  pc = 0x212148b; continue;
 }
 0x212137a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121382; continue;
 }
 0x212137f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2121382; continue;
 }
 0x2121382 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212137f; continue; }
  pc = 0x2121385; continue;
 }
 0x2121385 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21213dc; continue; }
  pc = 0x212139e; continue;
 }
 0x212139e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21213d3; continue; }
  pc = 0x21213c2; continue;
 }
 0x21213c2 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21213d3; continue;
 }
 0x21213d3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21213dc; continue;
 }
 0x21213dc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211d3; continue; }
  pc = 0x21213f3; continue;
 }
 0x21213f3 => {
  pc = 0x212148b; continue;
 }
 0x21213f8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121400; continue;
 }
 0x21213fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2121400; continue;
 }
 0x2121400 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21213fd; continue; }
  pc = 0x2121403; continue;
 }
 0x2121403 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121460; continue; }
  pc = 0x2121422; continue;
 }
 0x2121422 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121457; continue; }
  pc = 0x2121446; continue;
 }
 0x2121446 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121457; continue;
 }
 0x2121457 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121460; continue;
 }
 0x2121460 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x212147b; continue;
 }
 0x2121478 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x212147b; continue;
 }
 0x212147b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121478; continue; }
  pc = 0x212147e; continue;
 }
 0x212147e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21211d3; continue; }
  pc = 0x212148b; continue;
 }
 0x212148b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212148e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212148e); }
 let mut pc: u32 = 0x212148e;
 loop { match pc {
 0x212148e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21214de; continue; }
  pc = 0x21214a0; continue;
 }
 0x21214a0 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21214d5; continue; }
  pc = 0x21214c4; continue;
 }
 0x21214c4 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21214d5; continue;
 }
 0x21214d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21214de; continue;
 }
 0x21214de => {
  { let v: u32 = cpu.edi; cpu.w32(0x2300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2121500; continue; }
  pc = 0x21214f5; continue;
 }
 0x21214f5 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x2121500; continue;
 }
 0x2121500 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121523; continue;
 }
 0x2121507 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121523; continue;
 }
 0x2121523 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121507; continue; }
  pc = 0x2121526; continue;
 }
 0x2121526 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121500; continue; }
  pc = 0x212152f; continue;
 }
 0x212152f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121546; continue;
 }
 0x2121536 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121546; continue;
 }
 0x2121546 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121536; continue; }
  pc = 0x2121549; continue;
 }
 0x2121549 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x2121566; continue;
 }
 0x2121556 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121566; continue;
 }
 0x2121566 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121556; continue; }
  pc = 0x2121569; continue;
 }
 0x2121569 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21215be; continue; }
  pc = 0x2121580; continue;
 }
 0x2121580 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21215b5; continue; }
  pc = 0x21215a4; continue;
 }
 0x21215a4 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21215b5; continue;
 }
 0x21215b5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21215be; continue;
 }
 0x21215be => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x21215db; continue;
 }
 0x21215cb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21215db; continue;
 }
 0x21215db => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21215cb; continue; }
  pc = 0x21215de; continue;
 }
 0x21215de => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x21215e3; continue;
 }
 0x21215e3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121606; continue;
 }
 0x21215ea => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121606; continue;
 }
 0x2121606 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21215ea; continue; }
  pc = 0x2121609; continue;
 }
 0x2121609 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21215e3; continue; }
  pc = 0x2121612; continue;
 }
 0x2121612 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x2121629; continue;
 }
 0x2121619 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121629; continue;
 }
 0x2121629 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121619; continue; }
  pc = 0x212162c; continue;
 }
 0x212162c => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121681; continue; }
  pc = 0x2121643; continue;
 }
 0x2121643 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121678; continue; }
  pc = 0x2121667; continue;
 }
 0x2121667 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121678; continue;
 }
 0x2121678 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121681; continue;
 }
 0x2121681 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x212169e; continue;
 }
 0x212168e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212169e; continue;
 }
 0x212169e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212168e; continue; }
  pc = 0x21216a1; continue;
 }
 0x21216a1 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21216be; continue;
 }
 0x21216ae => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21216be; continue;
 }
 0x21216be => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21216ae; continue; }
  pc = 0x21216c1; continue;
 }
 0x21216c1 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x21216c6; continue;
 }
 0x21216c6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21216e9; continue;
 }
 0x21216cd => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21216e9; continue;
 }
 0x21216e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21216cd; continue; }
  pc = 0x21216ec; continue;
 }
 0x21216ec => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21216c6; continue; }
  pc = 0x21216f5; continue;
 }
 0x21216f5 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x212170c; continue;
 }
 0x21216fc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212170c; continue;
 }
 0x212170c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21216fc; continue; }
  pc = 0x212170f; continue;
 }
 0x212170f => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x212172c; continue;
 }
 0x212171c => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212172c; continue;
 }
 0x212172c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212171c; continue; }
  pc = 0x212172f; continue;
 }
 0x212172f => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121784; continue; }
  pc = 0x2121746; continue;
 }
 0x2121746 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212177b; continue; }
  pc = 0x212176a; continue;
 }
 0x212176a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212177b; continue;
 }
 0x212177b => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121784; continue;
 }
 0x2121784 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x21217a1; continue;
 }
 0x2121791 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21217a1; continue;
 }
 0x21217a1 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121791; continue; }
  pc = 0x21217a4; continue;
 }
 0x21217a4 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x21217a9; continue;
 }
 0x21217a9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21217cc; continue;
 }
 0x21217b0 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21217cc; continue;
 }
 0x21217cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21217b0; continue; }
  pc = 0x21217cf; continue;
 }
 0x21217cf => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21217a9; continue; }
  pc = 0x21217d8; continue;
 }
 0x21217d8 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x21217ef; continue;
 }
 0x21217df => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21217ef; continue;
 }
 0x21217ef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21217df; continue; }
  pc = 0x21217f2; continue;
 }
 0x21217f2 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121847; continue; }
  pc = 0x2121809; continue;
 }
 0x2121809 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212183e; continue; }
  pc = 0x212182d; continue;
 }
 0x212182d => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212183e; continue;
 }
 0x212183e => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121847; continue;
 }
 0x2121847 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x2121864; continue;
 }
 0x2121854 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121864; continue;
 }
 0x2121864 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121854; continue; }
  pc = 0x2121867; continue;
 }
 0x2121867 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121884; continue;
 }
 0x2121874 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121884; continue;
 }
 0x2121884 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121874; continue; }
  pc = 0x2121887; continue;
 }
 0x2121887 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x212188c; continue;
 }
 0x212188c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x21218af; continue;
 }
 0x2121893 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21218af; continue;
 }
 0x21218af => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121893; continue; }
  pc = 0x21218b2; continue;
 }
 0x21218b2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212188c; continue; }
  pc = 0x21218bb; continue;
 }
 0x21218bb => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121910; continue; }
  pc = 0x21218d2; continue;
 }
 0x21218d2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121907; continue; }
  pc = 0x21218f6; continue;
 }
 0x21218f6 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121907; continue;
 }
 0x2121907 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121910; continue;
 }
 0x2121910 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x212191b; continue;
 }
 0x212191b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x212193e; continue;
 }
 0x2121922 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x212193e; continue;
 }
 0x212193e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121922; continue; }
  pc = 0x2121941; continue;
 }
 0x2121941 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212191b; continue; }
  pc = 0x212194a; continue;
 }
 0x212194a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121961; continue;
 }
 0x2121951 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121961; continue;
 }
 0x2121961 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121951; continue; }
  pc = 0x2121964; continue;
 }
 0x2121964 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x2121981; continue;
 }
 0x2121971 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121981; continue;
 }
 0x2121981 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121971; continue; }
  pc = 0x2121984; continue;
 }
 0x2121984 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21219d9; continue; }
  pc = 0x212199b; continue;
 }
 0x212199b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21219d0; continue; }
  pc = 0x21219bf; continue;
 }
 0x21219bf => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21219d0; continue;
 }
 0x21219d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21219d9; continue;
 }
 0x21219d9 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x21219f6; continue;
 }
 0x21219e6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x21219f6; continue;
 }
 0x21219f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21219e6; continue; }
  pc = 0x21219f9; continue;
 }
 0x21219f9 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x21219fe; continue;
 }
 0x21219fe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121a21; continue;
 }
 0x2121a05 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121a21; continue;
 }
 0x2121a21 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121a05; continue; }
  pc = 0x2121a24; continue;
 }
 0x2121a24 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21219fe; continue; }
  pc = 0x2121a2d; continue;
 }
 0x2121a2d => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x2121a44; continue;
 }
 0x2121a34 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121a44; continue;
 }
 0x2121a44 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121a34; continue; }
  pc = 0x2121a47; continue;
 }
 0x2121a47 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121a9c; continue; }
  pc = 0x2121a5e; continue;
 }
 0x2121a5e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121a93; continue; }
  pc = 0x2121a82; continue;
 }
 0x2121a82 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121a93; continue;
 }
 0x2121a93 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121a9c; continue;
 }
 0x2121a9c => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x2121ab9; continue;
 }
 0x2121aa9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121ab9; continue;
 }
 0x2121ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121aa9; continue; }
  pc = 0x2121abc; continue;
 }
 0x2121abc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121ad9; continue;
 }
 0x2121ac9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121ad9; continue;
 }
 0x2121ad9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121ac9; continue; }
  pc = 0x2121adc; continue;
 }
 0x2121adc => {
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2121c05; continue; }
  pc = 0x2121aec; continue;
 }
 0x2121aec => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x2121af1; continue;
 }
 0x2121af1 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121b14; continue;
 }
 0x2121af8 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121b14; continue;
 }
 0x2121b14 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121af8; continue; }
  pc = 0x2121b17; continue;
 }
 0x2121b17 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121af1; continue; }
  pc = 0x2121b20; continue;
 }
 0x2121b20 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121b37; continue;
 }
 0x2121b27 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121b37; continue;
 }
 0x2121b37 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121b27; continue; }
  pc = 0x2121b3a; continue;
 }
 0x2121b3a => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x2121b57; continue;
 }
 0x2121b47 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121b57; continue;
 }
 0x2121b57 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121b47; continue; }
  pc = 0x2121b5a; continue;
 }
 0x2121b5a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121baf; continue; }
  pc = 0x2121b71; continue;
 }
 0x2121b71 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121ba6; continue; }
  pc = 0x2121b95; continue;
 }
 0x2121b95 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121ba6; continue;
 }
 0x2121ba6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121baf; continue;
 }
 0x2121baf => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x2121bcc; continue;
 }
 0x2121bbc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121bcc; continue;
 }
 0x2121bcc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121bbc; continue; }
  pc = 0x2121bcf; continue;
 }
 0x2121bcf => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x2121bd4; continue;
 }
 0x2121bd4 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121bf7; continue;
 }
 0x2121bdb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121bf7; continue;
 }
 0x2121bf7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121bdb; continue; }
  pc = 0x2121bfa; continue;
 }
 0x2121bfa => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121bd4; continue; }
  pc = 0x2121c03; continue;
 }
 0x2121c03 => {
  pc = 0x2121c39; continue;
 }
 0x2121c05 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x2121c0a; continue;
 }
 0x2121c0a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x2121c2d; continue;
 }
 0x2121c11 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2121c2d; continue;
 }
 0x2121c2d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121c11; continue; }
  pc = 0x2121c30; continue;
 }
 0x2121c30 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121c0a; continue; }
  pc = 0x2121c39; continue;
 }
 0x2121c39 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02121c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2121c3b); }
 let mut pc: u32 = 0x2121c3b;
 loop { match pc {
 0x2121c3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2121c5a; continue; }
  pc = 0x2121c57; continue;
 }
 0x2121c57 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x2121c5a; continue;
 }
 0x2121c5a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121cc4; continue; }
  pc = 0x2121c86; continue;
 }
 0x2121c86 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121cbb; continue; }
  pc = 0x2121caa; continue;
 }
 0x2121caa => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121cbb; continue;
 }
 0x2121cbb => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121cc4; continue;
 }
 0x2121cc4 => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x2121cff; continue;
 }
 0x2121cff => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x2121d2c; continue; }
  pc = 0x2121d07; continue;
 }
 0x2121d07 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121d1b; continue;
 }
 0x2121d0c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x2121d1b; continue;
 }
 0x2121d1b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121d0c; continue; }
  pc = 0x2121d1e; continue;
 }
 0x2121d1e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121cff; continue; }
  pc = 0x2121d27; continue;
 }
 0x2121d27 => {
  pc = 0x2121fff; continue;
 }
 0x2121d2c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x2121dfa; continue; }
  pc = 0x2121d3d; continue;
 }
 0x2121d3d => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2121d4b; continue;
 }
 0x2121d42 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121d4b; continue;
 }
 0x2121d4b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121d42; continue; }
  pc = 0x2121d4e; continue;
 }
 0x2121d4e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121da5; continue; }
  pc = 0x2121d67; continue;
 }
 0x2121d67 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121d9c; continue; }
  pc = 0x2121d8b; continue;
 }
 0x2121d8b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121d9c; continue;
 }
 0x2121d9c => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121da5; continue;
 }
 0x2121da5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2121dc5; continue;
 }
 0x2121dbc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121dc5; continue;
 }
 0x2121dc5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121dbc; continue; }
  pc = 0x2121dc8; continue;
 }
 0x2121dc8 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121ddf; continue;
 }
 0x2121dd6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121ddf; continue;
 }
 0x2121ddf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121dd6; continue; }
  pc = 0x2121de2; continue;
 }
 0x2121de2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121cff; continue; }
  pc = 0x2121df5; continue;
 }
 0x2121df5 => {
  pc = 0x2121fff; continue;
 }
 0x2121dfa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x2121f60; continue; }
  pc = 0x2121e08; continue;
 }
 0x2121e08 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x2121ed6; continue; }
  pc = 0x2121e11; continue;
 }
 0x2121e11 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121e1f; continue;
 }
 0x2121e16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121e1f; continue;
 }
 0x2121e1f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121e16; continue; }
  pc = 0x2121e22; continue;
 }
 0x2121e22 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2121e41; continue;
 }
 0x2121e38 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121e41; continue;
 }
 0x2121e41 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121e38; continue; }
  pc = 0x2121e44; continue;
 }
 0x2121e44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121e9b; continue; }
  pc = 0x2121e5d; continue;
 }
 0x2121e5d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121e92; continue; }
  pc = 0x2121e81; continue;
 }
 0x2121e81 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121e92; continue;
 }
 0x2121e92 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121e9b; continue;
 }
 0x2121e9b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2121ebb; continue;
 }
 0x2121eb2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121ebb; continue;
 }
 0x2121ebb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121eb2; continue; }
  pc = 0x2121ebe; continue;
 }
 0x2121ebe => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121cff; continue; }
  pc = 0x2121ed1; continue;
 }
 0x2121ed1 => {
  pc = 0x2121fff; continue;
 }
 0x2121ed6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121eea; continue;
 }
 0x2121edb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x2121eea; continue;
 }
 0x2121eea => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121edb; continue; }
  pc = 0x2121eed; continue;
 }
 0x2121eed => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121f44; continue; }
  pc = 0x2121f06; continue;
 }
 0x2121f06 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121f3b; continue; }
  pc = 0x2121f2a; continue;
 }
 0x2121f2a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121f3b; continue;
 }
 0x2121f3b => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121f44; continue;
 }
 0x2121f44 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121cff; continue; }
  pc = 0x2121f5b; continue;
 }
 0x2121f5b => {
  pc = 0x2121fff; continue;
 }
 0x2121f60 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121f6e; continue;
 }
 0x2121f65 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121f6e; continue;
 }
 0x2121f6e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121f65; continue; }
  pc = 0x2121f71; continue;
 }
 0x2121f71 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2121fce; continue; }
  pc = 0x2121f90; continue;
 }
 0x2121f90 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2121fc5; continue; }
  pc = 0x2121fb4; continue;
 }
 0x2121fb4 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2121fc5; continue;
 }
 0x2121fc5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2121fce; continue;
 }
 0x2121fce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2121fef; continue;
 }
 0x2121fe6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x2121fef; continue;
 }
 0x2121fef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121fe6; continue; }
  pc = 0x2121ff2; continue;
 }
 0x2121ff2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2121cff; continue; }
  pc = 0x2121fff; continue;
 }
 0x2121fff => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02122002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2122002); }
 let mut pc: u32 = 0x2122002;
 loop { match pc {
 0x2122002 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2122021; continue; }
  pc = 0x212201e; continue;
 }
 0x212201e => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x2122021; continue;
 }
 0x2122021 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212208b; continue; }
  pc = 0x212204d; continue;
 }
 0x212204d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2122082; continue; }
  pc = 0x2122071; continue;
 }
 0x2122071 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2122082; continue;
 }
 0x2122082 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212208b; continue;
 }
 0x212208b => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x21220c6; continue;
 }
 0x21220c6 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x21220e7; continue; }
  pc = 0x21220ce; continue;
 }
 0x21220ce => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21220d6; continue;
 }
 0x21220d3 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21220d6; continue;
 }
 0x21220d6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220d3; continue; }
  pc = 0x21220d9; continue;
 }
 0x21220d9 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220c6; continue; }
  pc = 0x21220e2; continue;
 }
 0x21220e2 => {
  pc = 0x212237e; continue;
 }
 0x21220e7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x21221a3; continue; }
  pc = 0x21220f8; continue;
 }
 0x21220f8 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2122100; continue;
 }
 0x21220fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2122100; continue;
 }
 0x2122100 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220fd; continue; }
  pc = 0x2122103; continue;
 }
 0x2122103 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212215a; continue; }
  pc = 0x212211c; continue;
 }
 0x212211c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2122151; continue; }
  pc = 0x2122140; continue;
 }
 0x2122140 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2122151; continue;
 }
 0x2122151 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212215a; continue;
 }
 0x212215a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2122174; continue;
 }
 0x2122171 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2122174; continue;
 }
 0x2122174 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122171; continue; }
  pc = 0x2122177; continue;
 }
 0x2122177 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2122188; continue;
 }
 0x2122185 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2122188; continue;
 }
 0x2122188 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122185; continue; }
  pc = 0x212218b; continue;
 }
 0x212218b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220c6; continue; }
  pc = 0x212219e; continue;
 }
 0x212219e => {
  pc = 0x212237e; continue;
 }
 0x21221a3 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x21222eb; continue; }
  pc = 0x21221b1; continue;
 }
 0x21221b1 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x212226d; continue; }
  pc = 0x21221ba; continue;
 }
 0x21221ba => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21221c2; continue;
 }
 0x21221bf => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21221c2; continue;
 }
 0x21221c2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21221bf; continue; }
  pc = 0x21221c5; continue;
 }
 0x21221c5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x21221de; continue;
 }
 0x21221db => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21221de; continue;
 }
 0x21221de => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21221db; continue; }
  pc = 0x21221e1; continue;
 }
 0x21221e1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2122238; continue; }
  pc = 0x21221fa; continue;
 }
 0x21221fa => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212222f; continue; }
  pc = 0x212221e; continue;
 }
 0x212221e => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212222f; continue;
 }
 0x212222f => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2122238; continue;
 }
 0x2122238 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2122252; continue;
 }
 0x212224f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2122252; continue;
 }
 0x2122252 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212224f; continue; }
  pc = 0x2122255; continue;
 }
 0x2122255 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220c6; continue; }
  pc = 0x2122268; continue;
 }
 0x2122268 => {
  pc = 0x212237e; continue;
 }
 0x212226d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2122275; continue;
 }
 0x2122272 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2122275; continue;
 }
 0x2122275 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122272; continue; }
  pc = 0x2122278; continue;
 }
 0x2122278 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21222cf; continue; }
  pc = 0x2122291; continue;
 }
 0x2122291 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21222c6; continue; }
  pc = 0x21222b5; continue;
 }
 0x21222b5 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21222c6; continue;
 }
 0x21222c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21222cf; continue;
 }
 0x21222cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220c6; continue; }
  pc = 0x21222e6; continue;
 }
 0x21222e6 => {
  pc = 0x212237e; continue;
 }
 0x21222eb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21222f3; continue;
 }
 0x21222f0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x21222f3; continue;
 }
 0x21222f3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21222f0; continue; }
  pc = 0x21222f6; continue;
 }
 0x21222f6 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2122353; continue; }
  pc = 0x2122315; continue;
 }
 0x2122315 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212234a; continue; }
  pc = 0x2122339; continue;
 }
 0x2122339 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212234a; continue;
 }
 0x212234a => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2122353; continue;
 }
 0x2122353 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x212236e; continue;
 }
 0x212236b => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x212236e; continue;
 }
 0x212236e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212236b; continue; }
  pc = 0x2122371; continue;
 }
 0x2122371 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21220c6; continue; }
  pc = 0x212237e; continue;
 }
 0x212237e => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02122381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2122381); }
 let mut pc: u32 = 0x2122381;
 loop { match pc {
 0x2122381 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21223d1; continue; }
  pc = 0x2122393; continue;
 }
 0x2122393 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21223c8; continue; }
  pc = 0x21223b7; continue;
 }
 0x21223b7 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21223c8; continue;
 }
 0x21223c8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21223d1; continue;
 }
 0x21223d1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x2300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x21223f3; continue; }
  pc = 0x21223e8; continue;
 }
 0x21223e8 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x21223f3; continue;
 }
 0x21223f3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21223fb, "Movq");
  cpu.unimplemented(0x2122402, "Movq");
  pc = 0x2122405; continue;
 }
 0x2122405 => {
  cpu.unimplemented(0x2122405, "Movq");
  cpu.unimplemented(0x2122409, "Movq");
  cpu.unimplemented(0x212240c, "Movq");
  cpu.unimplemented(0x212240f, "Movq");
  cpu.unimplemented(0x2122412, "Psrlq");
  cpu.unimplemented(0x2122416, "Psllq");
  cpu.unimplemented(0x212241a, "Pand");
  cpu.unimplemented(0x2122421, "Por");
  cpu.unimplemented(0x2122424, "Psrlq");
  cpu.unimplemented(0x2122428, "Pand");
  cpu.unimplemented(0x212242f, "Psrlq");
  cpu.unimplemented(0x2122433, "Paddw");
  cpu.unimplemented(0x2122436, "Movq");
  cpu.unimplemented(0x2122439, "Punpcklwd");
  cpu.unimplemented(0x212243c, "Punpckhwd");
  cpu.unimplemented(0x212243f, "Movq");
  cpu.unimplemented(0x2122442, "Pand");
  cpu.unimplemented(0x2122445, "Movq");
  cpu.unimplemented(0x212244c, "Movq");
  cpu.unimplemented(0x2122450, "Pand");
  cpu.unimplemented(0x2122453, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122460, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122405; continue; }
  pc = 0x2122466; continue;
 }
 0x2122466 => {
  cpu.unimplemented(0x2122466, "Punpcklwd");
  cpu.unimplemented(0x2122469, "Punpckhwd");
  cpu.unimplemented(0x212246c, "Movq");
  cpu.unimplemented(0x212246f, "Pand");
  cpu.unimplemented(0x2122472, "Movq");
  cpu.unimplemented(0x2122479, "Movq");
  cpu.unimplemented(0x212247d, "Pand");
  cpu.unimplemented(0x2122480, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21223f3; continue; }
  pc = 0x212249a; continue;
 }
 0x212249a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21224a2, "Movq");
  cpu.unimplemented(0x21224a9, "Movq");
  pc = 0x21224ac; continue;
 }
 0x21224ac => {
  cpu.unimplemented(0x21224ac, "Movq");
  cpu.unimplemented(0x21224b0, "Movq");
  cpu.unimplemented(0x21224b3, "Movq");
  cpu.unimplemented(0x21224b6, "Movq");
  cpu.unimplemented(0x21224b9, "Psrlq");
  cpu.unimplemented(0x21224bd, "Psllq");
  cpu.unimplemented(0x21224c1, "Pand");
  cpu.unimplemented(0x21224c8, "Por");
  cpu.unimplemented(0x21224cb, "Psrlq");
  cpu.unimplemented(0x21224cf, "Pand");
  cpu.unimplemented(0x21224d6, "Psrlq");
  cpu.unimplemented(0x21224da, "Paddw");
  cpu.unimplemented(0x21224dd, "Movq");
  cpu.unimplemented(0x21224e0, "Punpcklwd");
  cpu.unimplemented(0x21224e3, "Punpckhwd");
  cpu.unimplemented(0x21224e6, "Movq");
  cpu.unimplemented(0x21224e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21224f3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21224ac; continue; }
  pc = 0x21224f9; continue;
 }
 0x21224f9 => {
  cpu.unimplemented(0x21224f9, "Punpcklwd");
  cpu.unimplemented(0x21224fc, "Punpckhwd");
  cpu.unimplemented(0x21224ff, "Movq");
  cpu.unimplemented(0x2122502, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x212251a, "Movq");
  cpu.unimplemented(0x2122521, "Movq");
  pc = 0x2122524; continue;
 }
 0x2122524 => {
  cpu.unimplemented(0x2122524, "Movq");
  cpu.unimplemented(0x2122528, "Movq");
  cpu.unimplemented(0x212252b, "Movq");
  cpu.unimplemented(0x212252e, "Movq");
  cpu.unimplemented(0x2122531, "Psrlq");
  cpu.unimplemented(0x2122535, "Psllq");
  cpu.unimplemented(0x2122539, "Pand");
  cpu.unimplemented(0x2122540, "Por");
  cpu.unimplemented(0x2122543, "Psrlq");
  cpu.unimplemented(0x2122547, "Pand");
  cpu.unimplemented(0x212254e, "Psrlq");
  cpu.unimplemented(0x2122552, "Paddw");
  cpu.unimplemented(0x2122555, "Movq");
  cpu.unimplemented(0x2122558, "Punpcklwd");
  cpu.unimplemented(0x212255b, "Punpckhwd");
  cpu.unimplemented(0x212255e, "Movq");
  cpu.unimplemented(0x2122561, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x212256b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122524; continue; }
  pc = 0x2122571; continue;
 }
 0x2122571 => {
  cpu.unimplemented(0x2122571, "Punpcklwd");
  cpu.unimplemented(0x2122574, "Punpckhwd");
  cpu.unimplemented(0x2122577, "Movq");
  cpu.unimplemented(0x212257a, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21225d9; continue; }
  pc = 0x212259b; continue;
 }
 0x212259b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21225d0; continue; }
  pc = 0x21225bf; continue;
 }
 0x21225bf => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21225d0; continue;
 }
 0x21225d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21225d9; continue;
 }
 0x21225d9 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21225e7, "Movq");
  cpu.unimplemented(0x21225ee, "Movq");
  pc = 0x21225f1; continue;
 }
 0x21225f1 => {
  cpu.unimplemented(0x21225f1, "Movq");
  cpu.unimplemented(0x21225f5, "Movq");
  cpu.unimplemented(0x21225f8, "Movq");
  cpu.unimplemented(0x21225fb, "Movq");
  cpu.unimplemented(0x21225fe, "Psrlq");
  cpu.unimplemented(0x2122602, "Psllq");
  cpu.unimplemented(0x2122606, "Pand");
  cpu.unimplemented(0x212260d, "Por");
  cpu.unimplemented(0x2122610, "Psrlq");
  cpu.unimplemented(0x2122614, "Pand");
  cpu.unimplemented(0x212261b, "Psrlq");
  cpu.unimplemented(0x212261f, "Paddw");
  cpu.unimplemented(0x2122622, "Movq");
  cpu.unimplemented(0x2122625, "Punpcklwd");
  cpu.unimplemented(0x2122628, "Punpckhwd");
  cpu.unimplemented(0x212262b, "Movq");
  cpu.unimplemented(0x212262e, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122638, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21225f1; continue; }
  pc = 0x212263e; continue;
 }
 0x212263e => {
  cpu.unimplemented(0x212263e, "Punpcklwd");
  cpu.unimplemented(0x2122641, "Punpckhwd");
  cpu.unimplemented(0x2122644, "Movq");
  cpu.unimplemented(0x2122647, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2122656; continue;
 }
 0x2122656 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x212265e, "Movq");
  cpu.unimplemented(0x2122665, "Movq");
  pc = 0x2122668; continue;
 }
 0x2122668 => {
  cpu.unimplemented(0x2122668, "Movq");
  cpu.unimplemented(0x212266c, "Movq");
  cpu.unimplemented(0x212266f, "Movq");
  cpu.unimplemented(0x2122672, "Movq");
  cpu.unimplemented(0x2122675, "Psrlq");
  cpu.unimplemented(0x2122679, "Psllq");
  cpu.unimplemented(0x212267d, "Pand");
  cpu.unimplemented(0x2122684, "Por");
  cpu.unimplemented(0x2122687, "Psrlq");
  cpu.unimplemented(0x212268b, "Pand");
  cpu.unimplemented(0x2122692, "Psrlq");
  cpu.unimplemented(0x2122696, "Paddw");
  cpu.unimplemented(0x2122699, "Movq");
  cpu.unimplemented(0x212269c, "Punpcklwd");
  cpu.unimplemented(0x212269f, "Punpckhwd");
  cpu.unimplemented(0x21226a2, "Movq");
  cpu.unimplemented(0x21226a5, "Pand");
  cpu.unimplemented(0x21226a8, "Movq");
  cpu.unimplemented(0x21226af, "Movq");
  cpu.unimplemented(0x21226b3, "Pand");
  cpu.unimplemented(0x21226b6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21226c3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122668; continue; }
  pc = 0x21226c9; continue;
 }
 0x21226c9 => {
  cpu.unimplemented(0x21226c9, "Punpcklwd");
  cpu.unimplemented(0x21226cc, "Punpckhwd");
  cpu.unimplemented(0x21226cf, "Movq");
  cpu.unimplemented(0x21226d2, "Pand");
  cpu.unimplemented(0x21226d5, "Movq");
  cpu.unimplemented(0x21226dc, "Movq");
  cpu.unimplemented(0x21226e0, "Pand");
  cpu.unimplemented(0x21226e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122656; continue; }
  pc = 0x21226fd; continue;
 }
 0x21226fd => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122705, "Movq");
  cpu.unimplemented(0x212270c, "Movq");
  pc = 0x212270f; continue;
 }
 0x212270f => {
  cpu.unimplemented(0x212270f, "Movq");
  cpu.unimplemented(0x2122713, "Movq");
  cpu.unimplemented(0x2122716, "Movq");
  cpu.unimplemented(0x2122719, "Movq");
  cpu.unimplemented(0x212271c, "Psrlq");
  cpu.unimplemented(0x2122720, "Psllq");
  cpu.unimplemented(0x2122724, "Pand");
  cpu.unimplemented(0x212272b, "Por");
  cpu.unimplemented(0x212272e, "Psrlq");
  cpu.unimplemented(0x2122732, "Pand");
  cpu.unimplemented(0x2122739, "Psrlq");
  cpu.unimplemented(0x212273d, "Paddw");
  cpu.unimplemented(0x2122740, "Movq");
  cpu.unimplemented(0x2122743, "Punpcklwd");
  cpu.unimplemented(0x2122746, "Punpckhwd");
  cpu.unimplemented(0x2122749, "Movq");
  cpu.unimplemented(0x212274c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122756, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x212270f; continue; }
  pc = 0x212275c; continue;
 }
 0x212275c => {
  cpu.unimplemented(0x212275c, "Punpcklwd");
  cpu.unimplemented(0x212275f, "Punpckhwd");
  cpu.unimplemented(0x2122762, "Movq");
  cpu.unimplemented(0x2122765, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21227c4; continue; }
  pc = 0x2122786; continue;
 }
 0x2122786 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21227bb; continue; }
  pc = 0x21227aa; continue;
 }
 0x21227aa => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21227bb; continue;
 }
 0x21227bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21227c4; continue;
 }
 0x21227c4 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21227d2, "Movq");
  cpu.unimplemented(0x21227d9, "Movq");
  pc = 0x21227dc; continue;
 }
 0x21227dc => {
  cpu.unimplemented(0x21227dc, "Movq");
  cpu.unimplemented(0x21227e0, "Movq");
  cpu.unimplemented(0x21227e3, "Movq");
  cpu.unimplemented(0x21227e6, "Movq");
  cpu.unimplemented(0x21227e9, "Psrlq");
  cpu.unimplemented(0x21227ed, "Psllq");
  cpu.unimplemented(0x21227f1, "Pand");
  cpu.unimplemented(0x21227f8, "Por");
  cpu.unimplemented(0x21227fb, "Psrlq");
  cpu.unimplemented(0x21227ff, "Pand");
  cpu.unimplemented(0x2122806, "Psrlq");
  cpu.unimplemented(0x212280a, "Paddw");
  cpu.unimplemented(0x212280d, "Movq");
  cpu.unimplemented(0x2122810, "Punpcklwd");
  cpu.unimplemented(0x2122813, "Punpckhwd");
  cpu.unimplemented(0x2122816, "Movq");
  cpu.unimplemented(0x2122819, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122823, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21227dc; continue; }
  pc = 0x2122829; continue;
 }
 0x2122829 => {
  cpu.unimplemented(0x2122829, "Punpcklwd");
  cpu.unimplemented(0x212282c, "Punpckhwd");
  cpu.unimplemented(0x212282f, "Movq");
  cpu.unimplemented(0x2122832, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x212284a, "Movq");
  cpu.unimplemented(0x2122851, "Movq");
  pc = 0x2122854; continue;
 }
 0x2122854 => {
  cpu.unimplemented(0x2122854, "Movq");
  cpu.unimplemented(0x2122858, "Movq");
  cpu.unimplemented(0x212285b, "Movq");
  cpu.unimplemented(0x212285e, "Movq");
  cpu.unimplemented(0x2122861, "Psrlq");
  cpu.unimplemented(0x2122865, "Psllq");
  cpu.unimplemented(0x2122869, "Pand");
  cpu.unimplemented(0x2122870, "Por");
  cpu.unimplemented(0x2122873, "Psrlq");
  cpu.unimplemented(0x2122877, "Pand");
  cpu.unimplemented(0x212287e, "Psrlq");
  cpu.unimplemented(0x2122882, "Paddw");
  cpu.unimplemented(0x2122885, "Movq");
  cpu.unimplemented(0x2122888, "Punpcklwd");
  cpu.unimplemented(0x212288b, "Punpckhwd");
  cpu.unimplemented(0x212288e, "Movq");
  cpu.unimplemented(0x2122891, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x212289b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122854; continue; }
  pc = 0x21228a1; continue;
 }
 0x21228a1 => {
  cpu.unimplemented(0x21228a1, "Punpcklwd");
  cpu.unimplemented(0x21228a4, "Punpckhwd");
  cpu.unimplemented(0x21228a7, "Movq");
  cpu.unimplemented(0x21228aa, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x21228b9; continue;
 }
 0x21228b9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21228c1, "Movq");
  cpu.unimplemented(0x21228c8, "Movq");
  pc = 0x21228cb; continue;
 }
 0x21228cb => {
  cpu.unimplemented(0x21228cb, "Movq");
  cpu.unimplemented(0x21228cf, "Movq");
  cpu.unimplemented(0x21228d2, "Movq");
  cpu.unimplemented(0x21228d5, "Movq");
  cpu.unimplemented(0x21228d8, "Psrlq");
  cpu.unimplemented(0x21228dc, "Psllq");
  cpu.unimplemented(0x21228e0, "Pand");
  cpu.unimplemented(0x21228e7, "Por");
  cpu.unimplemented(0x21228ea, "Psrlq");
  cpu.unimplemented(0x21228ee, "Pand");
  cpu.unimplemented(0x21228f5, "Psrlq");
  cpu.unimplemented(0x21228f9, "Paddw");
  cpu.unimplemented(0x21228fc, "Movq");
  cpu.unimplemented(0x21228ff, "Punpcklwd");
  cpu.unimplemented(0x2122902, "Punpckhwd");
  cpu.unimplemented(0x2122905, "Movq");
  cpu.unimplemented(0x2122908, "Pand");
  cpu.unimplemented(0x212290b, "Movq");
  cpu.unimplemented(0x2122912, "Movq");
  cpu.unimplemented(0x2122916, "Pand");
  cpu.unimplemented(0x2122919, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122926, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21228cb; continue; }
  pc = 0x212292c; continue;
 }
 0x212292c => {
  cpu.unimplemented(0x212292c, "Punpcklwd");
  cpu.unimplemented(0x212292f, "Punpckhwd");
  cpu.unimplemented(0x2122932, "Movq");
  cpu.unimplemented(0x2122935, "Pand");
  cpu.unimplemented(0x2122938, "Movq");
  cpu.unimplemented(0x212293f, "Movq");
  cpu.unimplemented(0x2122943, "Pand");
  cpu.unimplemented(0x2122946, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21228b9; continue; }
  pc = 0x2122960; continue;
 }
 0x2122960 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122968, "Movq");
  cpu.unimplemented(0x212296f, "Movq");
  pc = 0x2122972; continue;
 }
 0x2122972 => {
  cpu.unimplemented(0x2122972, "Movq");
  cpu.unimplemented(0x2122976, "Movq");
  cpu.unimplemented(0x2122979, "Movq");
  cpu.unimplemented(0x212297c, "Movq");
  cpu.unimplemented(0x212297f, "Psrlq");
  cpu.unimplemented(0x2122983, "Psllq");
  cpu.unimplemented(0x2122987, "Pand");
  cpu.unimplemented(0x212298e, "Por");
  cpu.unimplemented(0x2122991, "Psrlq");
  cpu.unimplemented(0x2122995, "Pand");
  cpu.unimplemented(0x212299c, "Psrlq");
  cpu.unimplemented(0x21229a0, "Paddw");
  cpu.unimplemented(0x21229a3, "Movq");
  cpu.unimplemented(0x21229a6, "Punpcklwd");
  cpu.unimplemented(0x21229a9, "Punpckhwd");
  cpu.unimplemented(0x21229ac, "Movq");
  cpu.unimplemented(0x21229af, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21229b9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122972; continue; }
  pc = 0x21229bf; continue;
 }
 0x21229bf => {
  cpu.unimplemented(0x21229bf, "Punpcklwd");
  cpu.unimplemented(0x21229c2, "Punpckhwd");
  cpu.unimplemented(0x21229c5, "Movq");
  cpu.unimplemented(0x21229c8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21229e0, "Movq");
  cpu.unimplemented(0x21229e7, "Movq");
  pc = 0x21229ea; continue;
 }
 0x21229ea => {
  cpu.unimplemented(0x21229ea, "Movq");
  cpu.unimplemented(0x21229ee, "Movq");
  cpu.unimplemented(0x21229f1, "Movq");
  cpu.unimplemented(0x21229f4, "Movq");
  cpu.unimplemented(0x21229f7, "Psrlq");
  cpu.unimplemented(0x21229fb, "Psllq");
  cpu.unimplemented(0x21229ff, "Pand");
  cpu.unimplemented(0x2122a06, "Por");
  cpu.unimplemented(0x2122a09, "Psrlq");
  cpu.unimplemented(0x2122a0d, "Pand");
  cpu.unimplemented(0x2122a14, "Psrlq");
  cpu.unimplemented(0x2122a18, "Paddw");
  cpu.unimplemented(0x2122a1b, "Movq");
  cpu.unimplemented(0x2122a1e, "Punpcklwd");
  cpu.unimplemented(0x2122a21, "Punpckhwd");
  cpu.unimplemented(0x2122a24, "Movq");
  cpu.unimplemented(0x2122a27, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122a31, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21229ea; continue; }
  pc = 0x2122a37; continue;
 }
 0x2122a37 => {
  cpu.unimplemented(0x2122a37, "Punpcklwd");
  cpu.unimplemented(0x2122a3a, "Punpckhwd");
  cpu.unimplemented(0x2122a3d, "Movq");
  cpu.unimplemented(0x2122a40, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2122a9f; continue; }
  pc = 0x2122a61; continue;
 }
 0x2122a61 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2122a96; continue; }
  pc = 0x2122a85; continue;
 }
 0x2122a85 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2122a96; continue;
 }
 0x2122a96 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2122a9f; continue;
 }
 0x2122a9f => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122aad, "Movq");
  cpu.unimplemented(0x2122ab4, "Movq");
  pc = 0x2122ab7; continue;
 }
 0x2122ab7 => {
  cpu.unimplemented(0x2122ab7, "Movq");
  cpu.unimplemented(0x2122abb, "Movq");
  cpu.unimplemented(0x2122abe, "Movq");
  cpu.unimplemented(0x2122ac1, "Movq");
  cpu.unimplemented(0x2122ac4, "Psrlq");
  cpu.unimplemented(0x2122ac8, "Psllq");
  cpu.unimplemented(0x2122acc, "Pand");
  cpu.unimplemented(0x2122ad3, "Por");
  cpu.unimplemented(0x2122ad6, "Psrlq");
  cpu.unimplemented(0x2122ada, "Pand");
  cpu.unimplemented(0x2122ae1, "Psrlq");
  cpu.unimplemented(0x2122ae5, "Paddw");
  cpu.unimplemented(0x2122ae8, "Movq");
  cpu.unimplemented(0x2122aeb, "Punpcklwd");
  cpu.unimplemented(0x2122aee, "Punpckhwd");
  cpu.unimplemented(0x2122af1, "Movq");
  cpu.unimplemented(0x2122af4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122afe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122ab7; continue; }
  pc = 0x2122b04; continue;
 }
 0x2122b04 => {
  cpu.unimplemented(0x2122b04, "Punpcklwd");
  cpu.unimplemented(0x2122b07, "Punpckhwd");
  cpu.unimplemented(0x2122b0a, "Movq");
  cpu.unimplemented(0x2122b0d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2122b1c; continue;
 }
 0x2122b1c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122b24, "Movq");
  cpu.unimplemented(0x2122b2b, "Movq");
  pc = 0x2122b2e; continue;
 }
 0x2122b2e => {
  cpu.unimplemented(0x2122b2e, "Movq");
  cpu.unimplemented(0x2122b32, "Movq");
  cpu.unimplemented(0x2122b35, "Movq");
  cpu.unimplemented(0x2122b38, "Movq");
  cpu.unimplemented(0x2122b3b, "Psrlq");
  cpu.unimplemented(0x2122b3f, "Psllq");
  cpu.unimplemented(0x2122b43, "Pand");
  cpu.unimplemented(0x2122b4a, "Por");
  cpu.unimplemented(0x2122b4d, "Psrlq");
  cpu.unimplemented(0x2122b51, "Pand");
  cpu.unimplemented(0x2122b58, "Psrlq");
  cpu.unimplemented(0x2122b5c, "Paddw");
  cpu.unimplemented(0x2122b5f, "Movq");
  cpu.unimplemented(0x2122b62, "Punpcklwd");
  cpu.unimplemented(0x2122b65, "Punpckhwd");
  cpu.unimplemented(0x2122b68, "Movq");
  cpu.unimplemented(0x2122b6b, "Pand");
  cpu.unimplemented(0x2122b6e, "Movq");
  cpu.unimplemented(0x2122b75, "Movq");
  cpu.unimplemented(0x2122b79, "Pand");
  cpu.unimplemented(0x2122b7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122b89, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122b2e; continue; }
  pc = 0x2122b8f; continue;
 }
 0x2122b8f => {
  cpu.unimplemented(0x2122b8f, "Punpcklwd");
  cpu.unimplemented(0x2122b92, "Punpckhwd");
  cpu.unimplemented(0x2122b95, "Movq");
  cpu.unimplemented(0x2122b98, "Pand");
  cpu.unimplemented(0x2122b9b, "Movq");
  cpu.unimplemented(0x2122ba2, "Movq");
  cpu.unimplemented(0x2122ba6, "Pand");
  cpu.unimplemented(0x2122ba9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122b1c; continue; }
  pc = 0x2122bc3; continue;
 }
 0x2122bc3 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122bcb, "Movq");
  cpu.unimplemented(0x2122bd2, "Movq");
  pc = 0x2122bd5; continue;
 }
 0x2122bd5 => {
  cpu.unimplemented(0x2122bd5, "Movq");
  cpu.unimplemented(0x2122bd9, "Movq");
  cpu.unimplemented(0x2122bdc, "Movq");
  cpu.unimplemented(0x2122bdf, "Movq");
  cpu.unimplemented(0x2122be2, "Psrlq");
  cpu.unimplemented(0x2122be6, "Psllq");
  cpu.unimplemented(0x2122bea, "Pand");
  cpu.unimplemented(0x2122bf1, "Por");
  cpu.unimplemented(0x2122bf4, "Psrlq");
  cpu.unimplemented(0x2122bf8, "Pand");
  cpu.unimplemented(0x2122bff, "Psrlq");
  cpu.unimplemented(0x2122c03, "Paddw");
  cpu.unimplemented(0x2122c06, "Movq");
  cpu.unimplemented(0x2122c09, "Punpcklwd");
  cpu.unimplemented(0x2122c0c, "Punpckhwd");
  cpu.unimplemented(0x2122c0f, "Movq");
  cpu.unimplemented(0x2122c12, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122c1c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122bd5; continue; }
  pc = 0x2122c22; continue;
 }
 0x2122c22 => {
  cpu.unimplemented(0x2122c22, "Punpcklwd");
  cpu.unimplemented(0x2122c25, "Punpckhwd");
  cpu.unimplemented(0x2122c28, "Movq");
  cpu.unimplemented(0x2122c2b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2122c8a; continue; }
  pc = 0x2122c4c; continue;
 }
 0x2122c4c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2122c81; continue; }
  pc = 0x2122c70; continue;
 }
 0x2122c70 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2122c81; continue;
 }
 0x2122c81 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2122c8a; continue;
 }
 0x2122c8a => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122c98, "Movq");
  cpu.unimplemented(0x2122c9f, "Movq");
  pc = 0x2122ca2; continue;
 }
 0x2122ca2 => {
  cpu.unimplemented(0x2122ca2, "Movq");
  cpu.unimplemented(0x2122ca6, "Movq");
  cpu.unimplemented(0x2122ca9, "Movq");
  cpu.unimplemented(0x2122cac, "Movq");
  cpu.unimplemented(0x2122caf, "Psrlq");
  cpu.unimplemented(0x2122cb3, "Psllq");
  cpu.unimplemented(0x2122cb7, "Pand");
  cpu.unimplemented(0x2122cbe, "Por");
  cpu.unimplemented(0x2122cc1, "Psrlq");
  cpu.unimplemented(0x2122cc5, "Pand");
  cpu.unimplemented(0x2122ccc, "Psrlq");
  cpu.unimplemented(0x2122cd0, "Paddw");
  cpu.unimplemented(0x2122cd3, "Movq");
  cpu.unimplemented(0x2122cd6, "Punpcklwd");
  cpu.unimplemented(0x2122cd9, "Punpckhwd");
  cpu.unimplemented(0x2122cdc, "Movq");
  cpu.unimplemented(0x2122cdf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122ce9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122ca2; continue; }
  pc = 0x2122cef; continue;
 }
 0x2122cef => {
  cpu.unimplemented(0x2122cef, "Punpcklwd");
  cpu.unimplemented(0x2122cf2, "Punpckhwd");
  cpu.unimplemented(0x2122cf5, "Movq");
  cpu.unimplemented(0x2122cf8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122d10, "Movq");
  cpu.unimplemented(0x2122d17, "Movq");
  pc = 0x2122d1a; continue;
 }
 0x2122d1a => {
  cpu.unimplemented(0x2122d1a, "Movq");
  cpu.unimplemented(0x2122d1e, "Movq");
  cpu.unimplemented(0x2122d21, "Movq");
  cpu.unimplemented(0x2122d24, "Movq");
  cpu.unimplemented(0x2122d27, "Psrlq");
  cpu.unimplemented(0x2122d2b, "Psllq");
  cpu.unimplemented(0x2122d2f, "Pand");
  cpu.unimplemented(0x2122d36, "Por");
  cpu.unimplemented(0x2122d39, "Psrlq");
  cpu.unimplemented(0x2122d3d, "Pand");
  cpu.unimplemented(0x2122d44, "Psrlq");
  cpu.unimplemented(0x2122d48, "Paddw");
  cpu.unimplemented(0x2122d4b, "Movq");
  cpu.unimplemented(0x2122d4e, "Punpcklwd");
  cpu.unimplemented(0x2122d51, "Punpckhwd");
  cpu.unimplemented(0x2122d54, "Movq");
  cpu.unimplemented(0x2122d57, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122d61, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122d1a; continue; }
  pc = 0x2122d67; continue;
 }
 0x2122d67 => {
  cpu.unimplemented(0x2122d67, "Punpcklwd");
  cpu.unimplemented(0x2122d6a, "Punpckhwd");
  cpu.unimplemented(0x2122d6d, "Movq");
  cpu.unimplemented(0x2122d70, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2122d7f; continue;
 }
 0x2122d7f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122d87, "Movq");
  cpu.unimplemented(0x2122d8e, "Movq");
  pc = 0x2122d91; continue;
 }
 0x2122d91 => {
  cpu.unimplemented(0x2122d91, "Movq");
  cpu.unimplemented(0x2122d95, "Movq");
  cpu.unimplemented(0x2122d98, "Movq");
  cpu.unimplemented(0x2122d9b, "Movq");
  cpu.unimplemented(0x2122d9e, "Psrlq");
  cpu.unimplemented(0x2122da2, "Psllq");
  cpu.unimplemented(0x2122da6, "Pand");
  cpu.unimplemented(0x2122dad, "Por");
  cpu.unimplemented(0x2122db0, "Psrlq");
  cpu.unimplemented(0x2122db4, "Pand");
  cpu.unimplemented(0x2122dbb, "Psrlq");
  cpu.unimplemented(0x2122dbf, "Paddw");
  cpu.unimplemented(0x2122dc2, "Movq");
  cpu.unimplemented(0x2122dc5, "Punpcklwd");
  cpu.unimplemented(0x2122dc8, "Punpckhwd");
  cpu.unimplemented(0x2122dcb, "Movq");
  cpu.unimplemented(0x2122dce, "Pand");
  cpu.unimplemented(0x2122dd1, "Movq");
  cpu.unimplemented(0x2122dd8, "Movq");
  cpu.unimplemented(0x2122ddc, "Pand");
  cpu.unimplemented(0x2122ddf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122dec, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122d91; continue; }
  pc = 0x2122df2; continue;
 }
 0x2122df2 => {
  cpu.unimplemented(0x2122df2, "Punpcklwd");
  cpu.unimplemented(0x2122df5, "Punpckhwd");
  cpu.unimplemented(0x2122df8, "Movq");
  cpu.unimplemented(0x2122dfb, "Pand");
  cpu.unimplemented(0x2122dfe, "Movq");
  cpu.unimplemented(0x2122e05, "Movq");
  cpu.unimplemented(0x2122e09, "Pand");
  cpu.unimplemented(0x2122e0c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122d7f; continue; }
  pc = 0x2122e26; continue;
 }
 0x2122e26 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2122e7b; continue; }
  pc = 0x2122e3d; continue;
 }
 0x2122e3d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2122e72; continue; }
  pc = 0x2122e61; continue;
 }
 0x2122e61 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2122e72; continue;
 }
 0x2122e72 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2122e7b; continue;
 }
 0x2122e7b => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x2122e86; continue;
 }
 0x2122e86 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122e8e, "Movq");
  cpu.unimplemented(0x2122e95, "Movq");
  pc = 0x2122e98; continue;
 }
 0x2122e98 => {
  cpu.unimplemented(0x2122e98, "Movq");
  cpu.unimplemented(0x2122e9c, "Movq");
  cpu.unimplemented(0x2122e9f, "Movq");
  cpu.unimplemented(0x2122ea2, "Movq");
  cpu.unimplemented(0x2122ea5, "Psrlq");
  cpu.unimplemented(0x2122ea9, "Psllq");
  cpu.unimplemented(0x2122ead, "Pand");
  cpu.unimplemented(0x2122eb4, "Por");
  cpu.unimplemented(0x2122eb7, "Psrlq");
  cpu.unimplemented(0x2122ebb, "Pand");
  cpu.unimplemented(0x2122ec2, "Psrlq");
  cpu.unimplemented(0x2122ec6, "Paddw");
  cpu.unimplemented(0x2122ec9, "Movq");
  cpu.unimplemented(0x2122ecc, "Punpcklwd");
  cpu.unimplemented(0x2122ecf, "Punpckhwd");
  cpu.unimplemented(0x2122ed2, "Movq");
  cpu.unimplemented(0x2122ed5, "Pand");
  cpu.unimplemented(0x2122ed8, "Movq");
  cpu.unimplemented(0x2122edf, "Movq");
  cpu.unimplemented(0x2122ee3, "Pand");
  cpu.unimplemented(0x2122ee6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122ef3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122e98; continue; }
  pc = 0x2122ef9; continue;
 }
 0x2122ef9 => {
  cpu.unimplemented(0x2122ef9, "Punpcklwd");
  cpu.unimplemented(0x2122efc, "Punpckhwd");
  cpu.unimplemented(0x2122eff, "Movq");
  cpu.unimplemented(0x2122f02, "Pand");
  cpu.unimplemented(0x2122f05, "Movq");
  cpu.unimplemented(0x2122f0c, "Movq");
  cpu.unimplemented(0x2122f10, "Pand");
  cpu.unimplemented(0x2122f13, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2122e86; continue; }
  pc = 0x2122f2d; continue;
 }
 0x2122f2d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122f35, "Movq");
  cpu.unimplemented(0x2122f3c, "Movq");
  pc = 0x2122f3f; continue;
 }
 0x2122f3f => {
  cpu.unimplemented(0x2122f3f, "Movq");
  cpu.unimplemented(0x2122f43, "Movq");
  cpu.unimplemented(0x2122f46, "Movq");
  cpu.unimplemented(0x2122f49, "Movq");
  cpu.unimplemented(0x2122f4c, "Psrlq");
  cpu.unimplemented(0x2122f50, "Psllq");
  cpu.unimplemented(0x2122f54, "Pand");
  cpu.unimplemented(0x2122f5b, "Por");
  cpu.unimplemented(0x2122f5e, "Psrlq");
  cpu.unimplemented(0x2122f62, "Pand");
  cpu.unimplemented(0x2122f69, "Psrlq");
  cpu.unimplemented(0x2122f6d, "Paddw");
  cpu.unimplemented(0x2122f70, "Movq");
  cpu.unimplemented(0x2122f73, "Punpcklwd");
  cpu.unimplemented(0x2122f76, "Punpckhwd");
  cpu.unimplemented(0x2122f79, "Movq");
  cpu.unimplemented(0x2122f7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122f86, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122f3f; continue; }
  pc = 0x2122f8c; continue;
 }
 0x2122f8c => {
  cpu.unimplemented(0x2122f8c, "Punpcklwd");
  cpu.unimplemented(0x2122f8f, "Punpckhwd");
  cpu.unimplemented(0x2122f92, "Movq");
  cpu.unimplemented(0x2122f95, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2122fad, "Movq");
  cpu.unimplemented(0x2122fb4, "Movq");
  pc = 0x2122fb7; continue;
 }
 0x2122fb7 => {
  cpu.unimplemented(0x2122fb7, "Movq");
  cpu.unimplemented(0x2122fbb, "Movq");
  cpu.unimplemented(0x2122fbe, "Movq");
  cpu.unimplemented(0x2122fc1, "Movq");
  cpu.unimplemented(0x2122fc4, "Psrlq");
  cpu.unimplemented(0x2122fc8, "Psllq");
  cpu.unimplemented(0x2122fcc, "Pand");
  cpu.unimplemented(0x2122fd3, "Por");
  cpu.unimplemented(0x2122fd6, "Psrlq");
  cpu.unimplemented(0x2122fda, "Pand");
  cpu.unimplemented(0x2122fe1, "Psrlq");
  cpu.unimplemented(0x2122fe5, "Paddw");
  cpu.unimplemented(0x2122fe8, "Movq");
  cpu.unimplemented(0x2122feb, "Punpcklwd");
  cpu.unimplemented(0x2122fee, "Punpckhwd");
  cpu.unimplemented(0x2122ff1, "Movq");
  cpu.unimplemented(0x2122ff4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2122ffe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2122fb7; continue; }
  pc = 0x2123004; continue;
 }
 0x2123004 => {
  cpu.unimplemented(0x2123004, "Punpcklwd");
  cpu.unimplemented(0x2123007, "Punpckhwd");
  cpu.unimplemented(0x212300a, "Movq");
  cpu.unimplemented(0x212300d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212306c; continue; }
  pc = 0x212302e; continue;
 }
 0x212302e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2123063; continue; }
  pc = 0x2123052; continue;
 }
 0x2123052 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2123063; continue;
 }
 0x2123063 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212306c; continue;
 }
 0x212306c => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x212307a, "Movq");
  cpu.unimplemented(0x2123081, "Movq");
  pc = 0x2123084; continue;
 }
 0x2123084 => {
  cpu.unimplemented(0x2123084, "Movq");
  cpu.unimplemented(0x2123088, "Movq");
  cpu.unimplemented(0x212308b, "Movq");
  cpu.unimplemented(0x212308e, "Movq");
  cpu.unimplemented(0x2123091, "Psrlq");
  cpu.unimplemented(0x2123095, "Psllq");
  cpu.unimplemented(0x2123099, "Pand");
  cpu.unimplemented(0x21230a0, "Por");
  cpu.unimplemented(0x21230a3, "Psrlq");
  cpu.unimplemented(0x21230a7, "Pand");
  cpu.unimplemented(0x21230ae, "Psrlq");
  cpu.unimplemented(0x21230b2, "Paddw");
  cpu.unimplemented(0x21230b5, "Movq");
  cpu.unimplemented(0x21230b8, "Punpcklwd");
  cpu.unimplemented(0x21230bb, "Punpckhwd");
  cpu.unimplemented(0x21230be, "Movq");
  cpu.unimplemented(0x21230c1, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21230cb, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2123084; continue; }
  pc = 0x21230d1; continue;
 }
 0x21230d1 => {
  cpu.unimplemented(0x21230d1, "Punpcklwd");
  cpu.unimplemented(0x21230d4, "Punpckhwd");
  cpu.unimplemented(0x21230d7, "Movq");
  cpu.unimplemented(0x21230da, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x21230e9; continue;
 }
 0x21230e9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21230f1, "Movq");
  cpu.unimplemented(0x21230f8, "Movq");
  pc = 0x21230fb; continue;
 }
 0x21230fb => {
  cpu.unimplemented(0x21230fb, "Movq");
  cpu.unimplemented(0x21230ff, "Movq");
  cpu.unimplemented(0x2123102, "Movq");
  cpu.unimplemented(0x2123105, "Movq");
  cpu.unimplemented(0x2123108, "Psrlq");
  cpu.unimplemented(0x212310c, "Psllq");
  cpu.unimplemented(0x2123110, "Pand");
  cpu.unimplemented(0x2123117, "Por");
  cpu.unimplemented(0x212311a, "Psrlq");
  cpu.unimplemented(0x212311e, "Pand");
  cpu.unimplemented(0x2123125, "Psrlq");
  cpu.unimplemented(0x2123129, "Paddw");
  cpu.unimplemented(0x212312c, "Movq");
  cpu.unimplemented(0x212312f, "Punpcklwd");
  cpu.unimplemented(0x2123132, "Punpckhwd");
  cpu.unimplemented(0x2123135, "Movq");
  cpu.unimplemented(0x2123138, "Pand");
  cpu.unimplemented(0x212313b, "Movq");
  cpu.unimplemented(0x2123142, "Movq");
  cpu.unimplemented(0x2123146, "Pand");
  cpu.unimplemented(0x2123149, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x2123156, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21230fb; continue; }
  pc = 0x212315c; continue;
 }
 0x212315c => {
  cpu.unimplemented(0x212315c, "Punpcklwd");
  cpu.unimplemented(0x212315f, "Punpckhwd");
  cpu.unimplemented(0x2123162, "Movq");
  cpu.unimplemented(0x2123165, "Pand");
  cpu.unimplemented(0x2123168, "Movq");
  cpu.unimplemented(0x212316f, "Movq");
  cpu.unimplemented(0x2123173, "Pand");
  cpu.unimplemented(0x2123176, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21230e9; continue; }
  pc = 0x2123190; continue;
 }
 0x2123190 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123198, "Movq");
  cpu.unimplemented(0x212319f, "Movq");
  pc = 0x21231a2; continue;
 }
 0x21231a2 => {
  cpu.unimplemented(0x21231a2, "Movq");
  cpu.unimplemented(0x21231a6, "Movq");
  cpu.unimplemented(0x21231a9, "Movq");
  cpu.unimplemented(0x21231ac, "Movq");
  cpu.unimplemented(0x21231af, "Psrlq");
  cpu.unimplemented(0x21231b3, "Psllq");
  cpu.unimplemented(0x21231b7, "Pand");
  cpu.unimplemented(0x21231be, "Por");
  cpu.unimplemented(0x21231c1, "Psrlq");
  cpu.unimplemented(0x21231c5, "Pand");
  cpu.unimplemented(0x21231cc, "Psrlq");
  cpu.unimplemented(0x21231d0, "Paddw");
  cpu.unimplemented(0x21231d3, "Movq");
  cpu.unimplemented(0x21231d6, "Punpcklwd");
  cpu.unimplemented(0x21231d9, "Punpckhwd");
  cpu.unimplemented(0x21231dc, "Movq");
  cpu.unimplemented(0x21231df, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21231e9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21231a2; continue; }
  pc = 0x21231ef; continue;
 }
 0x21231ef => {
  cpu.unimplemented(0x21231ef, "Punpcklwd");
  cpu.unimplemented(0x21231f2, "Punpckhwd");
  cpu.unimplemented(0x21231f5, "Movq");
  cpu.unimplemented(0x21231f8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2123257; continue; }
  pc = 0x2123219; continue;
 }
 0x2123219 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212324e; continue; }
  pc = 0x212323d; continue;
 }
 0x212323d => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212324e; continue;
 }
 0x212324e => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2123257; continue;
 }
 0x2123257 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123265, "Movq");
  cpu.unimplemented(0x212326c, "Movq");
  pc = 0x212326f; continue;
 }
 0x212326f => {
  cpu.unimplemented(0x212326f, "Movq");
  cpu.unimplemented(0x2123273, "Movq");
  cpu.unimplemented(0x2123276, "Movq");
  cpu.unimplemented(0x2123279, "Movq");
  cpu.unimplemented(0x212327c, "Psrlq");
  cpu.unimplemented(0x2123280, "Psllq");
  cpu.unimplemented(0x2123284, "Pand");
  cpu.unimplemented(0x212328b, "Por");
  cpu.unimplemented(0x212328e, "Psrlq");
  cpu.unimplemented(0x2123292, "Pand");
  cpu.unimplemented(0x2123299, "Psrlq");
  cpu.unimplemented(0x212329d, "Paddw");
  cpu.unimplemented(0x21232a0, "Movq");
  cpu.unimplemented(0x21232a3, "Punpcklwd");
  cpu.unimplemented(0x21232a6, "Punpckhwd");
  cpu.unimplemented(0x21232a9, "Movq");
  cpu.unimplemented(0x21232ac, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21232b6, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x212326f; continue; }
  pc = 0x21232bc; continue;
 }
 0x21232bc => {
  cpu.unimplemented(0x21232bc, "Punpcklwd");
  cpu.unimplemented(0x21232bf, "Punpckhwd");
  cpu.unimplemented(0x21232c2, "Movq");
  cpu.unimplemented(0x21232c5, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21232dd, "Movq");
  cpu.unimplemented(0x21232e4, "Movq");
  pc = 0x21232e7; continue;
 }
 0x21232e7 => {
  cpu.unimplemented(0x21232e7, "Movq");
  cpu.unimplemented(0x21232eb, "Movq");
  cpu.unimplemented(0x21232ee, "Movq");
  cpu.unimplemented(0x21232f1, "Movq");
  cpu.unimplemented(0x21232f4, "Psrlq");
  cpu.unimplemented(0x21232f8, "Psllq");
  cpu.unimplemented(0x21232fc, "Pand");
  cpu.unimplemented(0x2123303, "Por");
  cpu.unimplemented(0x2123306, "Psrlq");
  cpu.unimplemented(0x212330a, "Pand");
  cpu.unimplemented(0x2123311, "Psrlq");
  cpu.unimplemented(0x2123315, "Paddw");
  cpu.unimplemented(0x2123318, "Movq");
  cpu.unimplemented(0x212331b, "Punpcklwd");
  cpu.unimplemented(0x212331e, "Punpckhwd");
  cpu.unimplemented(0x2123321, "Movq");
  cpu.unimplemented(0x2123324, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x212332e, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21232e7; continue; }
  pc = 0x2123334; continue;
 }
 0x2123334 => {
  cpu.unimplemented(0x2123334, "Punpcklwd");
  cpu.unimplemented(0x2123337, "Punpckhwd");
  cpu.unimplemented(0x212333a, "Movq");
  cpu.unimplemented(0x212333d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x212366b; continue; }
  pc = 0x2123357; continue;
 }
 0x2123357 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x212335c; continue;
 }
 0x212335c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123364, "Movq");
  cpu.unimplemented(0x212336b, "Movq");
  pc = 0x212336e; continue;
 }
 0x212336e => {
  cpu.unimplemented(0x212336e, "Movq");
  cpu.unimplemented(0x2123372, "Movq");
  cpu.unimplemented(0x2123375, "Movq");
  cpu.unimplemented(0x2123378, "Movq");
  cpu.unimplemented(0x212337b, "Psrlq");
  cpu.unimplemented(0x212337f, "Psllq");
  cpu.unimplemented(0x2123383, "Pand");
  cpu.unimplemented(0x212338a, "Por");
  cpu.unimplemented(0x212338d, "Psrlq");
  cpu.unimplemented(0x2123391, "Pand");
  cpu.unimplemented(0x2123398, "Psrlq");
  cpu.unimplemented(0x212339c, "Paddw");
  cpu.unimplemented(0x212339f, "Movq");
  cpu.unimplemented(0x21233a2, "Punpcklwd");
  cpu.unimplemented(0x21233a5, "Punpckhwd");
  cpu.unimplemented(0x21233a8, "Movq");
  cpu.unimplemented(0x21233ab, "Pand");
  cpu.unimplemented(0x21233ae, "Movq");
  cpu.unimplemented(0x21233b5, "Movq");
  cpu.unimplemented(0x21233b9, "Pand");
  cpu.unimplemented(0x21233bc, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21233c9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x212336e; continue; }
  pc = 0x21233cf; continue;
 }
 0x21233cf => {
  cpu.unimplemented(0x21233cf, "Punpcklwd");
  cpu.unimplemented(0x21233d2, "Punpckhwd");
  cpu.unimplemented(0x21233d5, "Movq");
  cpu.unimplemented(0x21233d8, "Pand");
  cpu.unimplemented(0x21233db, "Movq");
  cpu.unimplemented(0x21233e2, "Movq");
  cpu.unimplemented(0x21233e6, "Pand");
  cpu.unimplemented(0x21233e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212335c; continue; }
  pc = 0x2123403; continue;
 }
 0x2123403 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x212340b, "Movq");
  cpu.unimplemented(0x2123412, "Movq");
  pc = 0x2123415; continue;
 }
 0x2123415 => {
  cpu.unimplemented(0x2123415, "Movq");
  cpu.unimplemented(0x2123419, "Movq");
  cpu.unimplemented(0x212341c, "Movq");
  cpu.unimplemented(0x212341f, "Movq");
  cpu.unimplemented(0x2123422, "Psrlq");
  cpu.unimplemented(0x2123426, "Psllq");
  cpu.unimplemented(0x212342a, "Pand");
  cpu.unimplemented(0x2123431, "Por");
  cpu.unimplemented(0x2123434, "Psrlq");
  cpu.unimplemented(0x2123438, "Pand");
  cpu.unimplemented(0x212343f, "Psrlq");
  cpu.unimplemented(0x2123443, "Paddw");
  cpu.unimplemented(0x2123446, "Movq");
  cpu.unimplemented(0x2123449, "Punpcklwd");
  cpu.unimplemented(0x212344c, "Punpckhwd");
  cpu.unimplemented(0x212344f, "Movq");
  cpu.unimplemented(0x2123452, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x212345c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2123415; continue; }
  pc = 0x2123462; continue;
 }
 0x2123462 => {
  cpu.unimplemented(0x2123462, "Punpcklwd");
  cpu.unimplemented(0x2123465, "Punpckhwd");
  cpu.unimplemented(0x2123468, "Movq");
  cpu.unimplemented(0x212346b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123483, "Movq");
  cpu.unimplemented(0x212348a, "Movq");
  pc = 0x212348d; continue;
 }
 0x212348d => {
  cpu.unimplemented(0x212348d, "Movq");
  cpu.unimplemented(0x2123491, "Movq");
  cpu.unimplemented(0x2123494, "Movq");
  cpu.unimplemented(0x2123497, "Movq");
  cpu.unimplemented(0x212349a, "Psrlq");
  cpu.unimplemented(0x212349e, "Psllq");
  cpu.unimplemented(0x21234a2, "Pand");
  cpu.unimplemented(0x21234a9, "Por");
  cpu.unimplemented(0x21234ac, "Psrlq");
  cpu.unimplemented(0x21234b0, "Pand");
  cpu.unimplemented(0x21234b7, "Psrlq");
  cpu.unimplemented(0x21234bb, "Paddw");
  cpu.unimplemented(0x21234be, "Movq");
  cpu.unimplemented(0x21234c1, "Punpcklwd");
  cpu.unimplemented(0x21234c4, "Punpckhwd");
  cpu.unimplemented(0x21234c7, "Movq");
  cpu.unimplemented(0x21234ca, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21234d4, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x212348d; continue; }
  pc = 0x21234da; continue;
 }
 0x21234da => {
  cpu.unimplemented(0x21234da, "Punpcklwd");
  cpu.unimplemented(0x21234dd, "Punpckhwd");
  cpu.unimplemented(0x21234e0, "Movq");
  cpu.unimplemented(0x21234e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2123542; continue; }
  pc = 0x2123504; continue;
 }
 0x2123504 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2123539; continue; }
  pc = 0x2123528; continue;
 }
 0x2123528 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2123539; continue;
 }
 0x2123539 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2123542; continue;
 }
 0x2123542 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123550, "Movq");
  cpu.unimplemented(0x2123557, "Movq");
  pc = 0x212355a; continue;
 }
 0x212355a => {
  cpu.unimplemented(0x212355a, "Movq");
  cpu.unimplemented(0x212355e, "Movq");
  cpu.unimplemented(0x2123561, "Movq");
  cpu.unimplemented(0x2123564, "Movq");
  cpu.unimplemented(0x2123567, "Psrlq");
  cpu.unimplemented(0x212356b, "Psllq");
  cpu.unimplemented(0x212356f, "Pand");
  cpu.unimplemented(0x2123576, "Por");
  cpu.unimplemented(0x2123579, "Psrlq");
  cpu.unimplemented(0x212357d, "Pand");
  cpu.unimplemented(0x2123584, "Psrlq");
  cpu.unimplemented(0x2123588, "Paddw");
  cpu.unimplemented(0x212358b, "Movq");
  cpu.unimplemented(0x212358e, "Punpcklwd");
  cpu.unimplemented(0x2123591, "Punpckhwd");
  cpu.unimplemented(0x2123594, "Movq");
  cpu.unimplemented(0x2123597, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21235a1, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x212355a; continue; }
  pc = 0x21235a7; continue;
 }
 0x21235a7 => {
  cpu.unimplemented(0x21235a7, "Punpcklwd");
  cpu.unimplemented(0x21235aa, "Punpckhwd");
  cpu.unimplemented(0x21235ad, "Movq");
  cpu.unimplemented(0x21235b0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x21235bf; continue;
 }
 0x21235bf => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x21235c7, "Movq");
  cpu.unimplemented(0x21235ce, "Movq");
  pc = 0x21235d1; continue;
 }
 0x21235d1 => {
  cpu.unimplemented(0x21235d1, "Movq");
  cpu.unimplemented(0x21235d5, "Movq");
  cpu.unimplemented(0x21235d8, "Movq");
  cpu.unimplemented(0x21235db, "Movq");
  cpu.unimplemented(0x21235de, "Psrlq");
  cpu.unimplemented(0x21235e2, "Psllq");
  cpu.unimplemented(0x21235e6, "Pand");
  cpu.unimplemented(0x21235ed, "Por");
  cpu.unimplemented(0x21235f0, "Psrlq");
  cpu.unimplemented(0x21235f4, "Pand");
  cpu.unimplemented(0x21235fb, "Psrlq");
  cpu.unimplemented(0x21235ff, "Paddw");
  cpu.unimplemented(0x2123602, "Movq");
  cpu.unimplemented(0x2123605, "Punpcklwd");
  cpu.unimplemented(0x2123608, "Punpckhwd");
  cpu.unimplemented(0x212360b, "Movq");
  cpu.unimplemented(0x212360e, "Pand");
  cpu.unimplemented(0x2123611, "Movq");
  cpu.unimplemented(0x2123618, "Movq");
  cpu.unimplemented(0x212361c, "Pand");
  cpu.unimplemented(0x212361f, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x212362c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x21235d1; continue; }
  pc = 0x2123632; continue;
 }
 0x2123632 => {
  cpu.unimplemented(0x2123632, "Punpcklwd");
  cpu.unimplemented(0x2123635, "Punpckhwd");
  cpu.unimplemented(0x2123638, "Movq");
  cpu.unimplemented(0x212363b, "Pand");
  cpu.unimplemented(0x212363e, "Movq");
  cpu.unimplemented(0x2123645, "Movq");
  cpu.unimplemented(0x2123649, "Pand");
  cpu.unimplemented(0x212364c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21235bf; continue; }
  pc = 0x2123666; continue;
 }
 0x2123666 => {
  pc = 0x2123717; continue;
 }
 0x212366b => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x2123670; continue;
 }
 0x2123670 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x2123678, "Movq");
  cpu.unimplemented(0x212367f, "Movq");
  pc = 0x2123682; continue;
 }
 0x2123682 => {
  cpu.unimplemented(0x2123682, "Movq");
  cpu.unimplemented(0x2123686, "Movq");
  cpu.unimplemented(0x2123689, "Movq");
  cpu.unimplemented(0x212368c, "Movq");
  cpu.unimplemented(0x212368f, "Psrlq");
  cpu.unimplemented(0x2123693, "Psllq");
  cpu.unimplemented(0x2123697, "Pand");
  cpu.unimplemented(0x212369e, "Por");
  cpu.unimplemented(0x21236a1, "Psrlq");
  cpu.unimplemented(0x21236a5, "Pand");
  cpu.unimplemented(0x21236ac, "Psrlq");
  cpu.unimplemented(0x21236b0, "Paddw");
  cpu.unimplemented(0x21236b3, "Movq");
  cpu.unimplemented(0x21236b6, "Punpcklwd");
  cpu.unimplemented(0x21236b9, "Punpckhwd");
  cpu.unimplemented(0x21236bc, "Movq");
  cpu.unimplemented(0x21236bf, "Pand");
  cpu.unimplemented(0x21236c2, "Movq");
  cpu.unimplemented(0x21236c9, "Movq");
  cpu.unimplemented(0x21236cd, "Pand");
  cpu.unimplemented(0x21236d0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x21236dd, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2123682; continue; }
  pc = 0x21236e3; continue;
 }
 0x21236e3 => {
  cpu.unimplemented(0x21236e3, "Punpcklwd");
  cpu.unimplemented(0x21236e6, "Punpckhwd");
  cpu.unimplemented(0x21236e9, "Movq");
  cpu.unimplemented(0x21236ec, "Pand");
  cpu.unimplemented(0x21236ef, "Movq");
  cpu.unimplemented(0x21236f6, "Movq");
  cpu.unimplemented(0x21236fa, "Pand");
  cpu.unimplemented(0x21236fd, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123670; continue; }
  pc = 0x2123717; continue;
 }
 0x2123717 => {
  cpu.unimplemented(0x2123717, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212371b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212371b); }
 let mut pc: u32 = 0x212371b;
 loop { match pc {
 0x212371b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123720); }
 let mut pc: u32 = 0x2123720;
 loop { match pc {
 0x2123720 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123725); }
 let mut pc: u32 = 0x2123725;
 loop { match pc {
 0x2123725 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2251a4cu32); cpu.ecx = v; }
  pc = 0x2123734; continue;
 }
 0x2123734 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf64(0xfffffff8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x20u32); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x2123734; continue; }
  pc = 0x2123757; continue;
 }
 0x2123757 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123759); }
 let mut pc: u32 = 0x2123759;
 loop { match pc {
 0x2123759 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x212379c; continue;
 }
 0x212379c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212379c; continue; }
  pc = 0x21237ae; continue;
 }
 0x21237ae => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_021237b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x21237b1); }
 let mut pc: u32 = 0x21237b1;
 loop { match pc {
 0x21237b1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x21237f4; continue;
 }
 0x21237f4 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21237f4; continue; }
  pc = 0x2123805; continue;
 }
 0x2123805 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123808); }
 let mut pc: u32 = 0x2123808;
 loop { match pc {
 0x2123808 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 1u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2123823; continue; }
  pc = 0x212381d; continue;
 }
 0x212381d => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  pc = 0x2123823; continue;
 }
 0x2123823 => {
  { let v: u32 = cpu.r32(0x2251a68u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x212382b; continue;
 }
 0x212382b => {
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x2123852; continue;
 }
 0x2123836 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x2123852; continue;
 }
 0x2123852 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123836; continue; }
  pc = 0x2123855; continue;
 }
 0x2123855 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212382b; continue; }
  pc = 0x212385a; continue;
 }
 0x212385a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212385c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212385c); }
 let mut pc: u32 = 0x212385c;
 loop { match pc {
 0x212385c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x212387b; continue; }
  pc = 0x2123878; continue;
 }
 0x2123878 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x212387b; continue;
 }
 0x212387b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x21238a9; continue;
 }
 0x21238a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x21238bd; continue;
 }
 0x21238ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x21238bd; continue;
 }
 0x21238bd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21238ae; continue; }
  pc = 0x21238c0; continue;
 }
 0x21238c0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21238a9; continue; }
  pc = 0x21238c9; continue;
 }
 0x21238c9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_021238cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x21238cc); }
 let mut pc: u32 = 0x21238cc;
 loop { match pc {
 0x21238cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x21238eb; continue; }
  pc = 0x21238e8; continue;
 }
 0x21238e8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x21238eb; continue;
 }
 0x21238eb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x2123919; continue;
 }
 0x2123919 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x2123921; continue;
 }
 0x212391e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x2123921; continue;
 }
 0x2123921 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212391e; continue; }
  pc = 0x2123924; continue;
 }
 0x2123924 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123919; continue; }
  pc = 0x212392d; continue;
 }
 0x212392d => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123930); }
 let mut pc: u32 = 0x2123930;
 loop { match pc {
 0x2123930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300034u32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x230001cu32, v); }
  pc = 0x212394d; continue;
 }
 0x212394d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123960; continue; }
  pc = 0x212395a; continue;
 }
 0x212395a => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123960; continue;
 }
 0x2123960 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x21239a4; continue;
 }
 0x2123973 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x21239a4; continue;
 }
 0x21239a4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123973; continue; }
  pc = 0x21239ac; continue;
 }
 0x21239ac => {
  { let v: u32 = cpu.dec32(cpu.r32(0x230001cu32)); cpu.w32(0x230001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212394d; continue; }
  pc = 0x21239b4; continue;
 }
 0x21239b4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_021239b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x21239b6); }
 let mut pc: u32 = 0x21239b6;
 loop { match pc {
 0x21239b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x500u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300034u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300024u32, v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300028u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x230002cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2300030u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x230001cu32, v); }
  pc = 0x2123a26; continue;
 }
 0x2123a26 => {
  { let v: u32 = cpu.r32(0x230002cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123a3a; continue; }
  pc = 0x2123a34; continue;
 }
 0x2123a34 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123a3a; continue;
 }
 0x2123a3a => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2123a7e; continue;
 }
 0x2123a4d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2123a7e; continue;
 }
 0x2123a7e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123a4d; continue; }
  pc = 0x2123a86; continue;
 }
 0x2123a86 => {
  { let v: u32 = cpu.r32(0x2300030u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123a9a; continue; }
  pc = 0x2123a94; continue;
 }
 0x2123a94 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123a9a; continue;
 }
 0x2123a9a => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebx = v; }
  pc = 0x2123ab9; continue;
 }
 0x2123aa2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xfffffff8u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x2123ab9; continue;
 }
 0x2123ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123aa2; continue; }
  pc = 0x2123abc; continue;
 }
 0x2123abc => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x2300024u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x2300028u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x2300028u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x230001cu32)); cpu.w32(0x230001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123a26; continue; }
  pc = 0x2123ada; continue;
 }
 0x2123ada => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123ade); }
 let mut pc: u32 = 0x2123ade;
 loop { match pc {
 0x2123ade => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x2123b15; continue;
 }
 0x2123b15 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x2123b2c; continue;
 }
 0x2123b22 => {
  { cpu.lods(1, false); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x2123b2c; continue;
 }
 0x2123b2c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123b22; continue; }
  pc = 0x2123b2f; continue;
 }
 0x2123b2f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123b15; continue; }
  pc = 0x2123b3a; continue;
 }
 0x2123b3a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02123b3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2123b3d); }
 let mut pc: u32 = 0x2123b3d;
 loop { match pc {
 0x2123b3d => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2123b8f; continue; }
  pc = 0x2123b51; continue;
 }
 0x2123b51 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2123b86; continue; }
  pc = 0x2123b75; continue;
 }
 0x2123b75 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2123b86; continue;
 }
 0x2123b86 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2123b8f; continue;
 }
 0x2123b8f => {
  { let v: u32 = cpu.edi; cpu.w32(0x2300020u32, v); }
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300034u32, v); }
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2123bc9; continue; }
  pc = 0x2123bb8; continue;
 }
 0x2123bb8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x6400u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6400u32); cpu.edx = v; }
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x2123bc9; continue;
 }
 0x2123bc9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123bdd; continue; }
  pc = 0x2123bd7; continue;
 }
 0x2123bd7 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123bdd; continue;
 }
 0x2123bdd => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2123c7c; continue;
 }
 0x2123bf3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123c7c; continue;
 }
 0x2123c7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123bf3; continue; }
  pc = 0x2123c88; continue;
 }
 0x2123c88 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123bc9; continue; }
  pc = 0x2123c9c; continue;
 }
 0x2123c9c => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123cb0; continue; }
  pc = 0x2123caa; continue;
 }
 0x2123caa => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123cb0; continue;
 }
 0x2123cb0 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2123d15; continue;
 }
 0x2123cc8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123d15; continue;
 }
 0x2123d15 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123cc8; continue; }
  pc = 0x2123d1d; continue;
 }
 0x2123d1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2123d74; continue; }
  pc = 0x2123d36; continue;
 }
 0x2123d36 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2123d6b; continue; }
  pc = 0x2123d5a; continue;
 }
 0x2123d5a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2123d6b; continue;
 }
 0x2123d6b => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2123d74; continue;
 }
 0x2123d74 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123d8f; continue; }
  pc = 0x2123d89; continue;
 }
 0x2123d89 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123d8f; continue;
 }
 0x2123d8f => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2123df4; continue;
 }
 0x2123da7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123df4; continue;
 }
 0x2123df4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123da7; continue; }
  pc = 0x2123dfc; continue;
 }
 0x2123dfc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123e17; continue; }
  pc = 0x2123e11; continue;
 }
 0x2123e11 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123e17; continue;
 }
 0x2123e17 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2123e7c; continue;
 }
 0x2123e2f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123e7c; continue;
 }
 0x2123e7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123e2f; continue; }
  pc = 0x2123e84; continue;
 }
 0x2123e84 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x2123e8a; continue;
 }
 0x2123e8a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123e9e; continue; }
  pc = 0x2123e98; continue;
 }
 0x2123e98 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123e9e; continue;
 }
 0x2123e9e => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2123f3d; continue;
 }
 0x2123eb4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123f3d; continue;
 }
 0x2123f3d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123eb4; continue; }
  pc = 0x2123f49; continue;
 }
 0x2123f49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2123e8a; continue; }
  pc = 0x2123f5d; continue;
 }
 0x2123f5d => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2123f71; continue; }
  pc = 0x2123f6b; continue;
 }
 0x2123f6b => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2123f71; continue;
 }
 0x2123f71 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2123fd6; continue;
 }
 0x2123f89 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2123fd6; continue;
 }
 0x2123fd6 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2123f89; continue; }
  pc = 0x2123fde; continue;
 }
 0x2123fde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2124035; continue; }
  pc = 0x2123ff7; continue;
 }
 0x2123ff7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212402c; continue; }
  pc = 0x212401b; continue;
 }
 0x212401b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212402c; continue;
 }
 0x212402c => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2124035; continue;
 }
 0x2124035 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124050; continue; }
  pc = 0x212404a; continue;
 }
 0x212404a => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124050; continue;
 }
 0x2124050 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x21240b5; continue;
 }
 0x2124068 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21240b5; continue;
 }
 0x21240b5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124068; continue; }
  pc = 0x21240bd; continue;
 }
 0x21240bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21240d8; continue; }
  pc = 0x21240d2; continue;
 }
 0x21240d2 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21240d8; continue;
 }
 0x21240d8 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x212413d; continue;
 }
 0x21240f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x212413d; continue;
 }
 0x212413d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21240f0; continue; }
  pc = 0x2124145; continue;
 }
 0x2124145 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x212414b; continue;
 }
 0x212414b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x212415f; continue; }
  pc = 0x2124159; continue;
 }
 0x2124159 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x212415f; continue;
 }
 0x212415f => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x21241fe; continue;
 }
 0x2124175 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21241fe; continue;
 }
 0x21241fe => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124175; continue; }
  pc = 0x212420a; continue;
 }
 0x212420a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212414b; continue; }
  pc = 0x212421e; continue;
 }
 0x212421e => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124232; continue; }
  pc = 0x212422c; continue;
 }
 0x212422c => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124232; continue;
 }
 0x2124232 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2124297; continue;
 }
 0x212424a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2124297; continue;
 }
 0x2124297 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x212424a; continue; }
  pc = 0x212429f; continue;
 }
 0x212429f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21242ba; continue; }
  pc = 0x21242b4; continue;
 }
 0x21242b4 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21242ba; continue;
 }
 0x21242ba => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x212431f; continue;
 }
 0x21242d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x212431f; continue;
 }
 0x212431f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21242d2; continue; }
  pc = 0x2124327; continue;
 }
 0x2124327 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212437e; continue; }
  pc = 0x2124340; continue;
 }
 0x2124340 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124375; continue; }
  pc = 0x2124364; continue;
 }
 0x2124364 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124375; continue;
 }
 0x2124375 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212437e; continue;
 }
 0x212437e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x21247b5; continue; }
  pc = 0x212438f; continue;
 }
 0x212438f => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21243a9; continue; }
  pc = 0x21243a3; continue;
 }
 0x21243a3 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21243a9; continue;
 }
 0x21243a9 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x212440e; continue;
 }
 0x21243c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x212440e; continue;
 }
 0x212440e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21243c1; continue; }
  pc = 0x2124416; continue;
 }
 0x2124416 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x212441c; continue;
 }
 0x212441c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124430; continue; }
  pc = 0x212442a; continue;
 }
 0x212442a => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124430; continue;
 }
 0x2124430 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x21244cf; continue;
 }
 0x2124446 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21244cf; continue;
 }
 0x21244cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124446; continue; }
  pc = 0x21244db; continue;
 }
 0x21244db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212441c; continue; }
  pc = 0x21244ef; continue;
 }
 0x21244ef => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124503; continue; }
  pc = 0x21244fd; continue;
 }
 0x21244fd => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124503; continue;
 }
 0x2124503 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2124568; continue;
 }
 0x212451b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2124568; continue;
 }
 0x2124568 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x212451b; continue; }
  pc = 0x2124570; continue;
 }
 0x2124570 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x212458b; continue; }
  pc = 0x2124585; continue;
 }
 0x2124585 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x212458b; continue;
 }
 0x212458b => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x21245f0; continue;
 }
 0x21245a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21245f0; continue;
 }
 0x21245f0 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21245a3; continue; }
  pc = 0x21245f8; continue;
 }
 0x21245f8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212464f; continue; }
  pc = 0x2124611; continue;
 }
 0x2124611 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124646; continue; }
  pc = 0x2124635; continue;
 }
 0x2124635 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124646; continue;
 }
 0x2124646 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212464f; continue;
 }
 0x212464f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x212466a; continue; }
  pc = 0x2124664; continue;
 }
 0x2124664 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x212466a; continue;
 }
 0x212466a => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x21246cf; continue;
 }
 0x2124682 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21246cf; continue;
 }
 0x21246cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124682; continue; }
  pc = 0x21246d7; continue;
 }
 0x21246d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x21246dd; continue;
 }
 0x21246dd => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21246f1; continue; }
  pc = 0x21246eb; continue;
 }
 0x21246eb => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21246f1; continue;
 }
 0x21246f1 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124790; continue;
 }
 0x2124707 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2124790; continue;
 }
 0x2124790 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124707; continue; }
  pc = 0x212479c; continue;
 }
 0x212479c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21246dd; continue; }
  pc = 0x21247b0; continue;
 }
 0x21247b0 => {
  pc = 0x2124915; continue;
 }
 0x21247b5 => {
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21247cf; continue; }
  pc = 0x21247c9; continue;
 }
 0x21247c9 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21247cf; continue;
 }
 0x21247cf => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x2124834; continue;
 }
 0x21247e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x2124834; continue;
 }
 0x2124834 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21247e7; continue; }
  pc = 0x212483c; continue;
 }
 0x212483c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2eu32; cpu.ebp = v; }
  pc = 0x2124842; continue;
 }
 0x2124842 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124856; continue; }
  pc = 0x2124850; continue;
 }
 0x2124850 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124856; continue;
 }
 0x2124856 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x21248f5; continue;
 }
 0x212486c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x280u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x281u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x282u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x283u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x282u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x21248f5; continue;
 }
 0x21248f5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x212486c; continue; }
  pc = 0x2124901; continue;
 }
 0x2124901 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2124842; continue; }
  pc = 0x2124915; continue;
 }
 0x2124915 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02124918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2124918); }
 let mut pc: u32 = 0x2124918;
 loop { match pc {
 0x2124918 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x2251000u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300034u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2124947; continue; }
  pc = 0x2124944; continue;
 }
 0x2124944 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x2124947; continue;
 }
 0x2124947 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21249b6; continue; }
  pc = 0x2124978; continue;
 }
 0x2124978 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21249ad; continue; }
  pc = 0x212499c; continue;
 }
 0x212499c => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21249ad; continue;
 }
 0x21249ad => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21249b6; continue;
 }
 0x21249b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x21249ee; continue;
 }
 0x21249ee => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x2124a85; continue; }
  pc = 0x21249fa; continue;
 }
 0x21249fa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124a0d; continue; }
  pc = 0x2124a07; continue;
 }
 0x2124a07 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124a0d; continue;
 }
 0x2124a0d => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124a64; continue;
 }
 0x2124a1f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124a64; continue;
 }
 0x2124a64 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124a1f; continue; }
  pc = 0x2124a6c; continue;
 }
 0x2124a6c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21249ee; continue; }
  pc = 0x2124a80; continue;
 }
 0x2124a80 => {
  pc = 0x2125004; continue;
 }
 0x2124a85 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x2124c2b; continue; }
  pc = 0x2124a96; continue;
 }
 0x2124a96 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124aa8; continue; }
  pc = 0x2124aa2; continue;
 }
 0x2124aa2 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124aa8; continue;
 }
 0x2124aa8 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124ae2; continue;
 }
 0x2124aba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124ae2; continue;
 }
 0x2124ae2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124aba; continue; }
  pc = 0x2124aea; continue;
 }
 0x2124aea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2124b43; continue; }
  pc = 0x2124b05; continue;
 }
 0x2124b05 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124b3a; continue; }
  pc = 0x2124b29; continue;
 }
 0x2124b29 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124b3a; continue;
 }
 0x2124b3a => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2124b43; continue;
 }
 0x2124b43 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124b67; continue; }
  pc = 0x2124b61; continue;
 }
 0x2124b61 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124b67; continue;
 }
 0x2124b67 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124ba1; continue;
 }
 0x2124b79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124ba1; continue;
 }
 0x2124ba1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124b79; continue; }
  pc = 0x2124ba9; continue;
 }
 0x2124ba9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124bca; continue; }
  pc = 0x2124bc4; continue;
 }
 0x2124bc4 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124bca; continue;
 }
 0x2124bca => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124c04; continue;
 }
 0x2124bdc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124c04; continue;
 }
 0x2124c04 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124bdc; continue; }
  pc = 0x2124c0c; continue;
 }
 0x2124c0c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21249ee; continue; }
  pc = 0x2124c26; continue;
 }
 0x2124c26 => {
  pc = 0x2125004; continue;
 }
 0x2124c2b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x2124ed1; continue; }
  pc = 0x2124c39; continue;
 }
 0x2124c39 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x2124ddf; continue; }
  pc = 0x2124c42; continue;
 }
 0x2124c42 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124c55; continue; }
  pc = 0x2124c4f; continue;
 }
 0x2124c4f => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124c55; continue;
 }
 0x2124c55 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124c8f; continue;
 }
 0x2124c67 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124c8f; continue;
 }
 0x2124c8f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124c67; continue; }
  pc = 0x2124c97; continue;
 }
 0x2124c97 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124cbf; continue; }
  pc = 0x2124cb9; continue;
 }
 0x2124cb9 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124cbf; continue;
 }
 0x2124cbf => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124cf9; continue;
 }
 0x2124cd1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124cf9; continue;
 }
 0x2124cf9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124cd1; continue; }
  pc = 0x2124d01; continue;
 }
 0x2124d01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2124d5a; continue; }
  pc = 0x2124d1c; continue;
 }
 0x2124d1c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124d51; continue; }
  pc = 0x2124d40; continue;
 }
 0x2124d40 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124d51; continue;
 }
 0x2124d51 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2124d5a; continue;
 }
 0x2124d5a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124d7e; continue; }
  pc = 0x2124d78; continue;
 }
 0x2124d78 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124d7e; continue;
 }
 0x2124d7e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124db8; continue;
 }
 0x2124d90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124db8; continue;
 }
 0x2124db8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124d90; continue; }
  pc = 0x2124dc0; continue;
 }
 0x2124dc0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21249ee; continue; }
  pc = 0x2124dda; continue;
 }
 0x2124dda => {
  pc = 0x2125004; continue;
 }
 0x2124ddf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124df2; continue; }
  pc = 0x2124dec; continue;
 }
 0x2124dec => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124df2; continue;
 }
 0x2124df2 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124e49; continue;
 }
 0x2124e04 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x280u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124e49; continue;
 }
 0x2124e49 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124e04; continue; }
  pc = 0x2124e51; continue;
 }
 0x2124e51 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2124eaa; continue; }
  pc = 0x2124e6c; continue;
 }
 0x2124e6c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124ea1; continue; }
  pc = 0x2124e90; continue;
 }
 0x2124e90 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124ea1; continue;
 }
 0x2124ea1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2124eaa; continue;
 }
 0x2124eaa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21249ee; continue; }
  pc = 0x2124ecc; continue;
 }
 0x2124ecc => {
  pc = 0x2125004; continue;
 }
 0x2124ed1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124ee4; continue; }
  pc = 0x2124ede; continue;
 }
 0x2124ede => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124ee4; continue;
 }
 0x2124ee4 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124f1e; continue;
 }
 0x2124ef6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124f1e; continue;
 }
 0x2124f1e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124ef6; continue; }
  pc = 0x2124f26; continue;
 }
 0x2124f26 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2124f85; continue; }
  pc = 0x2124f47; continue;
 }
 0x2124f47 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2124f7c; continue; }
  pc = 0x2124f6b; continue;
 }
 0x2124f6b => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2124f7c; continue;
 }
 0x2124f7c => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2124f85; continue;
 }
 0x2124f85 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2124fae; continue; }
  pc = 0x2124fa8; continue;
 }
 0x2124fa8 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2124fae; continue;
 }
 0x2124fae => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x2300018u32, v); }
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  pc = 0x2124fe8; continue;
 }
 0x2124fc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xfffffff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | ((cpu.ecx >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x2124fe8; continue;
 }
 0x2124fe8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2124fc0; continue; }
  pc = 0x2124ff0; continue;
 }
 0x2124ff0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21249ee; continue; }
  pc = 0x2125004; continue;
 }
 0x2125004 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125007); }
 let mut pc: u32 = 0x2125007;
 loop { match pc {
 0x2125007 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x2251a20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x2125026; continue; }
  pc = 0x2125023; continue;
 }
 0x2125023 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x2125026; continue;
 }
 0x2125026 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x2251a6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x2251a48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x2251a60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212508e; continue; }
  pc = 0x2125050; continue;
 }
 0x2125050 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125085; continue; }
  pc = 0x2125074; continue;
 }
 0x2125074 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125085; continue;
 }
 0x2125085 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212508e; continue;
 }
 0x212508e => {
  { let v: u32 = cpu.r32(0x2251a48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x2251a24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x21250c8; continue;
 }
 0x21250c8 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x2251a64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x21250fb; continue; }
  pc = 0x21250d0; continue;
 }
 0x21250d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x21250e9; continue;
 }
 0x21250de => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x21250e9; continue;
 }
 0x21250e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250de; continue; }
  pc = 0x21250ec; continue;
 }
 0x21250ec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250c8; continue; }
  pc = 0x21250f6; continue;
 }
 0x21250f6 => {
  pc = 0x2125427; continue;
 }
 0x21250fb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x21251e8; continue; }
  pc = 0x212510c; continue;
 }
 0x212510c => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x2125124; continue;
 }
 0x2125119 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x2125124; continue;
 }
 0x2125124 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125119; continue; }
  pc = 0x2125127; continue;
 }
 0x2125127 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212517f; continue; }
  pc = 0x2125141; continue;
 }
 0x2125141 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125176; continue; }
  pc = 0x2125165; continue;
 }
 0x2125165 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125176; continue;
 }
 0x2125176 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212517f; continue;
 }
 0x212517f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x21251a9; continue;
 }
 0x212519e => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x21251a9; continue;
 }
 0x21251a9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212519e; continue; }
  pc = 0x21251ac; continue;
 }
 0x21251ac => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x21251cc; continue;
 }
 0x21251c1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x21251cc; continue;
 }
 0x21251cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21251c1; continue; }
  pc = 0x21251cf; continue;
 }
 0x21251cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250c8; continue; }
  pc = 0x21251e3; continue;
 }
 0x21251e3 => {
  pc = 0x2125427; continue;
 }
 0x21251e8 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x2125373; continue; }
  pc = 0x21251f6; continue;
 }
 0x21251f6 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x21252e3; continue; }
  pc = 0x21251ff; continue;
 }
 0x21251ff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x2125218; continue;
 }
 0x212520d => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x2125218; continue;
 }
 0x2125218 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212520d; continue; }
  pc = 0x212521b; continue;
 }
 0x212521b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2251a24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x2125242; continue;
 }
 0x2125237 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x2125242; continue;
 }
 0x2125242 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125237; continue; }
  pc = 0x2125245; continue;
 }
 0x2125245 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212529d; continue; }
  pc = 0x212525f; continue;
 }
 0x212525f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125294; continue; }
  pc = 0x2125283; continue;
 }
 0x2125283 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125294; continue;
 }
 0x2125294 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212529d; continue;
 }
 0x212529d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2251a24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x21252c7; continue;
 }
 0x21252bc => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x21252c7; continue;
 }
 0x21252c7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21252bc; continue; }
  pc = 0x21252ca; continue;
 }
 0x21252ca => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250c8; continue; }
  pc = 0x21252de; continue;
 }
 0x21252de => {
  pc = 0x2125427; continue;
 }
 0x21252e3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x21252fc; continue;
 }
 0x21252f1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x21252fc; continue;
 }
 0x21252fc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x21252f1; continue; }
  pc = 0x21252ff; continue;
 }
 0x21252ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2125357; continue; }
  pc = 0x2125319; continue;
 }
 0x2125319 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x212534e; continue; }
  pc = 0x212533d; continue;
 }
 0x212533d => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x212534e; continue;
 }
 0x212534e => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2125357; continue;
 }
 0x2125357 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250c8; continue; }
  pc = 0x212536e; continue;
 }
 0x212536e => {
  pc = 0x2125427; continue;
 }
 0x2125373 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x212538c; continue;
 }
 0x2125381 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x212538c; continue;
 }
 0x212538c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125381; continue; }
  pc = 0x212538f; continue;
 }
 0x212538f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21253ea; continue; }
  pc = 0x21253ac; continue;
 }
 0x21253ac => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21253e1; continue; }
  pc = 0x21253d0; continue;
 }
 0x21253d0 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21253e1; continue;
 }
 0x21253e1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21253ea; continue;
 }
 0x21253ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x2251a24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x2251a48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2251a80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x2125416; continue;
 }
 0x212540b => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x2125416; continue;
 }
 0x2125416 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x212540b; continue; }
  pc = 0x2125419; continue;
 }
 0x2125419 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x21250c8; continue; }
  pc = 0x2125427; continue;
 }
 0x2125427 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0212542a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x212542a); }
 let mut pc: u32 = 0x212542a;
 loop { match pc {
 0x212542a => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212547c; continue; }
  pc = 0x212543e; continue;
 }
 0x212543e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125473; continue; }
  pc = 0x2125462; continue;
 }
 0x2125462 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125473; continue;
 }
 0x2125473 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212547c; continue;
 }
 0x212547c => {
  { let v: u32 = cpu.edi; cpu.w32(0x2300020u32, v); }
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300034u32, v); }
  { let v: u32 = 0x66u32; cpu.ebp = v; }
  pc = 0x2125499; continue;
 }
 0x2125499 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21254ad; continue; }
  pc = 0x21254a7; continue;
 }
 0x21254a7 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21254ad; continue;
 }
 0x21254ad => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x21254f1; continue;
 }
 0x21254c0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x21254f1; continue;
 }
 0x21254f1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21254c0; continue; }
  pc = 0x21254f9; continue;
 }
 0x21254f9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125499; continue; }
  pc = 0x21254fd; continue;
 }
 0x21254fd => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125511; continue; }
  pc = 0x212550b; continue;
 }
 0x212550b => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125511; continue;
 }
 0x2125511 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125555; continue;
 }
 0x2125524 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125555; continue;
 }
 0x2125555 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125524; continue; }
  pc = 0x212555d; continue;
 }
 0x212555d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21255b4; continue; }
  pc = 0x2125576; continue;
 }
 0x2125576 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21255ab; continue; }
  pc = 0x212559a; continue;
 }
 0x212559a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21255ab; continue;
 }
 0x21255ab => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21255b4; continue;
 }
 0x21255b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21255cf; continue; }
  pc = 0x21255c9; continue;
 }
 0x21255c9 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21255cf; continue;
 }
 0x21255cf => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125613; continue;
 }
 0x21255e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125613; continue;
 }
 0x2125613 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21255e2; continue; }
  pc = 0x212561b; continue;
 }
 0x212561b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125630; continue; }
  pc = 0x212562a; continue;
 }
 0x212562a => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125630; continue;
 }
 0x2125630 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125674; continue;
 }
 0x2125643 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125674; continue;
 }
 0x2125674 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125643; continue; }
  pc = 0x212567c; continue;
 }
 0x212567c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x2125682; continue;
 }
 0x2125682 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125696; continue; }
  pc = 0x2125690; continue;
 }
 0x2125690 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125696; continue;
 }
 0x2125696 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x21256da; continue;
 }
 0x21256a9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x21256da; continue;
 }
 0x21256da => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21256a9; continue; }
  pc = 0x21256e2; continue;
 }
 0x21256e2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125682; continue; }
  pc = 0x21256e6; continue;
 }
 0x21256e6 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21256fa; continue; }
  pc = 0x21256f4; continue;
 }
 0x21256f4 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21256fa; continue;
 }
 0x21256fa => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x212573e; continue;
 }
 0x212570d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x212573e; continue;
 }
 0x212573e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x212570d; continue; }
  pc = 0x2125746; continue;
 }
 0x2125746 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x212579d; continue; }
  pc = 0x212575f; continue;
 }
 0x212575f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125794; continue; }
  pc = 0x2125783; continue;
 }
 0x2125783 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125794; continue;
 }
 0x2125794 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x212579d; continue;
 }
 0x212579d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21257b8; continue; }
  pc = 0x21257b2; continue;
 }
 0x21257b2 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21257b8; continue;
 }
 0x21257b8 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x21257fc; continue;
 }
 0x21257cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x21257fc; continue;
 }
 0x21257fc => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21257cb; continue; }
  pc = 0x2125804; continue;
 }
 0x2125804 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125819; continue; }
  pc = 0x2125813; continue;
 }
 0x2125813 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125819; continue;
 }
 0x2125819 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x212585d; continue;
 }
 0x212582c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x212585d; continue;
 }
 0x212585d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x212582c; continue; }
  pc = 0x2125865; continue;
 }
 0x2125865 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x212586b; continue;
 }
 0x212586b => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x212587f; continue; }
  pc = 0x2125879; continue;
 }
 0x2125879 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x212587f; continue;
 }
 0x212587f => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x21258c3; continue;
 }
 0x2125892 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x21258c3; continue;
 }
 0x21258c3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125892; continue; }
  pc = 0x21258cb; continue;
 }
 0x21258cb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x212586b; continue; }
  pc = 0x21258cf; continue;
 }
 0x21258cf => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x21258e3; continue; }
  pc = 0x21258dd; continue;
 }
 0x21258dd => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x21258e3; continue;
 }
 0x21258e3 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125927; continue;
 }
 0x21258f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125927; continue;
 }
 0x2125927 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x21258f6; continue; }
  pc = 0x212592f; continue;
 }
 0x212592f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125944; continue; }
  pc = 0x212593e; continue;
 }
 0x212593e => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125944; continue;
 }
 0x2125944 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125988; continue;
 }
 0x2125957 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125988; continue;
 }
 0x2125988 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125957; continue; }
  pc = 0x2125990; continue;
 }
 0x2125990 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x21259e7; continue; }
  pc = 0x21259a9; continue;
 }
 0x21259a9 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x21259de; continue; }
  pc = 0x21259cd; continue;
 }
 0x21259cd => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x21259de; continue;
 }
 0x21259de => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x21259e7; continue;
 }
 0x21259e7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125a02; continue; }
  pc = 0x21259fc; continue;
 }
 0x21259fc => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125a02; continue;
 }
 0x2125a02 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125a46; continue;
 }
 0x2125a15 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125a46; continue;
 }
 0x2125a46 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125a15; continue; }
  pc = 0x2125a4e; continue;
 }
 0x2125a4e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x2125a54; continue;
 }
 0x2125a54 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125a68; continue; }
  pc = 0x2125a62; continue;
 }
 0x2125a62 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125a68; continue;
 }
 0x2125a68 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125aac; continue;
 }
 0x2125a7b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125aac; continue;
 }
 0x2125aac => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125a7b; continue; }
  pc = 0x2125ab4; continue;
 }
 0x2125ab4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125a54; continue; }
  pc = 0x2125ab8; continue;
 }
 0x2125ab8 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125acc; continue; }
  pc = 0x2125ac6; continue;
 }
 0x2125ac6 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125acc; continue;
 }
 0x2125acc => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125b10; continue;
 }
 0x2125adf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125b10; continue;
 }
 0x2125b10 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125adf; continue; }
  pc = 0x2125b18; continue;
 }
 0x2125b18 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125b2d; continue; }
  pc = 0x2125b27; continue;
 }
 0x2125b27 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125b2d; continue;
 }
 0x2125b2d => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125b71; continue;
 }
 0x2125b40 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125b71; continue;
 }
 0x2125b71 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125b40; continue; }
  pc = 0x2125b79; continue;
 }
 0x2125b79 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x2251a60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x2251abau32)); }
  if cpu.zf { pc = 0x2125bd0; continue; }
  pc = 0x2125b92; continue;
 }
 0x2125b92 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x2250fe4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125bc7; continue; }
  pc = 0x2125bb6; continue;
 }
 0x2125bb6 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125bc7; continue;
 }
 0x2125bc7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x2250fe4u32, v); }
  pc = 0x2125bd0; continue;
 }
 0x2125bd0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125beb; continue; }
  pc = 0x2125be5; continue;
 }
 0x2125be5 => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125beb; continue;
 }
 0x2125beb => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125c2f; continue;
 }
 0x2125bfe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125c2f; continue;
 }
 0x2125c2f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125bfe; continue; }
  pc = 0x2125c37; continue;
 }
 0x2125c37 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x46u32; cpu.ebp = v; }
  pc = 0x2125c3d; continue;
 }
 0x2125c3d => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x2300034u32)); }
  if cpu.cf { pc = 0x2125c51; continue; }
  pc = 0x2125c4b; continue;
 }
 0x2125c4b => {
  { let v: u32 = 0x2251000u32; cpu.edx = v; }
  pc = 0x2125c51; continue;
 }
 0x2125c51 => {
  { let v: u32 = cpu.r32(0x21740b8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2300018u32, v); }
  pc = 0x2125c95; continue;
 }
 0x2125c64 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0x3fff8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx & 0x3fff8u32; cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | cpu.r8(0x1u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x2125c95; continue;
 }
 0x2125c95 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x2300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x2125c64; continue; }
  pc = 0x2125c9d; continue;
 }
 0x2125c9d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2125c3d; continue; }
  pc = 0x2125ca1; continue;
 }
 0x2125ca1 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125ca4); }
 let mut pc: u32 = 0x2125ca4;
 loop { match pc {
 0x2125ca4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125cac); }
 let mut pc: u32 = 0x2125cac;
 loop { match pc {
 0x2125cac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125cb4); }
 let mut pc: u32 = 0x2125cb4;
 loop { match pc {
 0x2125cb4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x2125cbd; continue;
 }
 0x2125cbd => {
  pc = 0x2125cbf; continue;
 }
 0x2125cbf => {
  pc = 0x2125cc1; continue;
 }
 0x2125cc1 => {
  {  }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x2125cc3; continue;
 }
 0x2125cc3 => {
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x2125ccc; continue;
 }
 0x2125ccc => {
  pc = 0x2125cce; continue;
 }
 0x2125cce => {
  pc = 0x2125cd0; continue;
 }
 0x2125cd0 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x2125cde; continue;
 }
 0x2125cde => {
  pc = 0x2125ce0; continue;
 }
 0x2125ce0 => {
  pc = 0x2125ce2; continue;
 }
 0x2125ce2 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x2125cf0; continue;
 }
 0x2125cf0 => {
  pc = 0x2125cf2; continue;
 }
 0x2125cf2 => {
  pc = 0x2125cf4; continue;
 }
 0x2125cf4 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x2125cc3; continue; }
  pc = 0x2125cfc; continue;
 }
 0x2125cfc => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125d26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125d26); }
 let mut pc: u32 = 0x2125d26;
 loop { match pc {
 0x2125d26 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x2125d34; continue; }
  pc = 0x2125d2f; continue;
 }
 0x2125d2f => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x2125d2f; continue; }
  pc = 0x2125d34; continue;
 }
 0x2125d34 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x2125d34; continue; }
  pc = 0x2125d39; continue;
 }
 0x2125d39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125d3a); }
 let mut pc: u32 = 0x2125d3a;
 loop { match pc {
 0x2125d3a => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2251ac0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02125d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2125d47); }
 let mut pc: u32 = 0x2125d47;
 loop { match pc {
 0x2125d47 => {
  { let v: u16 = cpu.r16(0x2251ac0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x2251abau32), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x2125d83; continue; }
  pc = 0x2125d57; continue;
 }
 0x2125d57 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2251abau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x2251abcu32), 0xffffu16); }
  if cpu.zf { pc = 0x2125d83; continue; }
  pc = 0x2125d72; continue;
 }
 0x2125d72 => {
  { let v: u16 = cpu.r16(0x2251abau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x2125d83; continue;
 }
 0x2125d83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02130000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2130000); }
 let mut pc: u32 = 0x2130000;
 loop { match pc {
 0x2130000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x2130003; continue;
 }
 0x2130003 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x2130011; continue; }
  pc = 0x213000e; continue;
 }
 0x213000e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x2130011; continue;
 }
 0x2130011 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x2130003; continue; }
  pc = 0x2130020; continue;
 }
 0x2130020 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02130022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2130022); }
 let mut pc: u32 = 0x2130022;
 loop { match pc {
 0x2130022 => {
  { let v = cpu.ebp; cpu.push(v); }
  {  }
  { let v: u16 = 0x201u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  {  }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02130038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x2130038); }
 let mut pc: u32 = 0x2130038;
 loop { match pc {
 0x2130038 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  {  }
  { let v: u16 = 0x201u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x2250f9cu32, v); }
  pc = 0x2130064; continue;
 }
 0x2130064 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x2250f9cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x2130080; continue; }
  pc = 0x2130071; continue;
 }
 0x2130071 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2250f9cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2250f9cu32, v); }
  pc = 0x2130080; continue;
 }
 0x2130080 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x2130085; continue; }
  pc = 0x2130084; continue;
 }
 0x2130084 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x2130085; continue;
 }
 0x2130085 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x213008a; continue; }
  pc = 0x2130089; continue;
 }
 0x2130089 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x213008a; continue;
 }
 0x213008a => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x213008f; continue; }
  pc = 0x213008e; continue;
 }
 0x213008e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x213008f; continue;
 }
 0x213008f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x2130094; continue; }
  pc = 0x2130093; continue;
 }
 0x2130093 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x2130094; continue;
 }
 0x2130094 => {
  { cpu.and8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x2130064; continue; }
  pc = 0x2130098; continue;
 }
 0x2130098 => {
  {  }
  { let v: u32 = cpu.esi; cpu.w32(0x2250fc0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x2250fbcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x2250fc4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x2250fa0u32, v); }
  { let v: u8 = cpu.r8(0x2250f9cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2250f9cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02140000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2140000); } r_02140000(cpu, 0x2140000); }
pub fn f_02140008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2140008); } r_02140000(cpu, 0x2140008); }
fn r_02140000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x2140000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x2140008; continue;
 }
 0x2140008 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.esi; cpu.set_segment(3, v as u16); }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if !cpu.zf { pc = 0x2140014; continue; }
  pc = 0x2140011; continue;
 }
 0x2140011 => {
  cpu.flat_ds(0x2140011); cpu.push(0x2140016); cpu.call(0x1970051); if cpu.returned_to != 0x2140016 { return; }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esi); cpu.w32(0x3u32.wrapping_add(cpu.ebp), v); }
  pc = 0x2140019; continue;
 }
 0x2140014 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x214001c; continue; }
  pc = 0x2140019; continue;
 }
 0x2140019 => {
  cpu.flat_ds(0x2140019); cpu.push(0x214001e); cpu.call(0x1970065); if cpu.returned_to != 0x214001e { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x3u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x2140021; continue;
 }
 0x214001c => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x2140024; continue; }
  pc = 0x2140021; continue;
 }
 0x2140021 => {
  cpu.flat_ds(0x2140021); cpu.push(0x2140026); cpu.call(0x1970073); if cpu.returned_to != 0x2140026 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  pc = 0x2140029; continue;
 }
 0x2140024 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x214002c; continue; }
  pc = 0x2140029; continue;
 }
 0x2140029 => {
  cpu.flat_ds(0x2140029); cpu.push(0x214002e); cpu.call(0x1970079); if cpu.returned_to != 0x214002e { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x75u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x2140039; continue;
 }
 0x214002c => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x2140034; continue; }
  pc = 0x2140031; continue;
 }
 0x2140031 => {
  cpu.flat_ds(0x2140031); cpu.push(0x2140036); cpu.call(0x197008d); if cpu.returned_to != 0x2140036 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x55e80375u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x2140041; continue;
 }
 0x2140034 => {
  { cpu.sub32(cpu.edi, 0x5u32); }
  if !cpu.zf { pc = 0x214003c; continue; }
  pc = 0x2140039; continue;
 }
 0x2140039 => {
  cpu.flat_ds(0x2140039); cpu.push(0x214003e); cpu.call(0x1970093); if cpu.returned_to != 0x214003e { return; }
  { cpu.push(cpu.segments[0] as u32); }
  if !cpu.zf { pc = 0x2140044; continue; }
  pc = 0x2140041; continue;
 }
 0x214003c => {
  { cpu.sub32(cpu.edi, 0x6u32); }
  if !cpu.zf { pc = 0x2140044; continue; }
  pc = 0x2140041; continue;
 }
 0x2140041 => {
  cpu.flat_ds(0x2140041); cpu.push(0x2140046); cpu.call(0x19700a3); if cpu.returned_to != 0x2140046 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if !cpu.zf { pc = 0x214004c; continue; }
  pc = 0x2140049; continue;
 }
 0x2140044 => {
  { cpu.sub32(cpu.edi, 0x7u32); }
  if !cpu.zf { pc = 0x214004c; continue; }
  pc = 0x2140049; continue;
 }
 0x2140049 => {
  cpu.flat_ds(0x2140049); cpu.push(0x214004e); cpu.call(0x93300b1); if cpu.returned_to != 0x214004e { return; }
  cpu.flat_ds(0x214004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x214004c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x214004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0214004f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x214004f); }
 let mut pc: u32 = 0x214004f;
 loop { match pc {
 0x214004f => {
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
pub fn f_02150000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2150000); } r_02150000(cpu, 0x2150000); }
pub fn f_02150002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2150002); } r_02150000(cpu, 0x2150002); }
fn r_02150000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x2150000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x2150002; continue;
 }
 0x2150002 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x2150042; continue; }
  pc = 0x2150009; continue;
 }
 0x2150009 => {
  { let v: u32 = 0x66670007u32; cpu.eax = v; }
  { let v: u32 = 0xffffe000u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 16u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x2150042; continue; }
  pc = 0x215001f; continue;
 }
 0x215001f => {
  {  }
  { let v = cpu.ebx; cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x8dfb2000u32; cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x500e0fffu32); cpu.eax = v; }
  { let v = (cpu.edi as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x2150036 => {
  {  }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.ebp as u16); cpu.esp = (cpu.esp & 0xffff0000) | v as u32; }
  {  }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x2150042 => {
  cpu.unimplemented(0x2150042, "Int3");
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x2150036; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_02000000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2000000); } r_02000000(cpu, 0x2000000); }
pub fn f_02000001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x2000001); } r_02000000(cpu, 0x2000001); }
fn r_02000000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x2000000 => {
  cpu.unimplemented(0x2000000, "Int3");
  pc = 0x2000001; continue;
 }
 0x2000001 => {
  cpu.poll(); pc = 0x2000000; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0204b853(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x204b853); } r_0204b853(cpu, 0x204b853); }
pub fn f_0204b866(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x204b866); } r_0204b853(cpu, 0x204b866); }
fn r_0204b853(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x204b84d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  pc = 0x204b853; continue;
 }
 0x204b853 => {
  if cpu.zf { pc = 0x204b865; continue; }
  pc = 0x204b855; continue;
 }
 0x204b855 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x204b84d; continue; }
  pc = 0x204b865; continue;
 }
 0x204b865 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x204b866; continue;
 }
 0x204b866 => {
  { let v: u32 = 0x21669c8u32; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x204b874; continue;
 }
 0x204b874 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x204b88c; continue; }
  pc = 0x204b87c; continue;
 }
 0x204b87c => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x204b874; continue; }
  pc = 0x204b88c; continue;
 }
 0x204b88c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x204b89a; continue;
 }
 0x204b89a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x204b8b2; continue; }
  pc = 0x204b8a2; continue;
 }
 0x204b8a2 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x204b89a; continue; }
  pc = 0x204b8b2; continue;
 }
 0x204b8b2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x204b8ba); f_020a28dc(cpu); if cpu.returned_to != 0x204b8ba { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x204b8e9; continue; }
  pc = 0x204b8bf; continue;
 }
 0x204b8bf => {
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x7100u32; cpu.ebx = v; }
  { let v: u32 = 0x2042c68u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2173340u32); cpu.edx = v; }
  { let v: u32 = 0x21dbb98u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x20000u32); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.push(0x204b8e5); f_020a2794(cpu); if cpu.returned_to != 0x204b8e5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x204b8f3; continue; }
  pc = 0x204b8e9; continue;
 }
 0x204b8e9 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x204b964; continue;
 }
 0x204b8f3 => {
  { let v: u32 = cpu.r32(0x2173340u32); cpu.eax = v; }
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20000u32); cpu.eax = v; }
  cpu.push(0x204b906); f_020a2012(cpu); if cpu.returned_to != 0x204b906 { return; }
  { let v: u32 = 0x21669d4u32; cpu.edx = v; }
  cpu.push(0x204b910); f_020a20a7(cpu); if cpu.returned_to != 0x204b910 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x204b933; continue; }
  pc = 0x204b914; continue;
 }
 0x204b914 => {
  { let v = 0x21669e4u32; cpu.push(v); }
  { let v = 0x2166a10u32; cpu.push(v); }
  cpu.push(0x204b923); f_020a3d38(cpu); if cpu.returned_to != 0x204b923 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x204b933 => {
  { let v: u32 = 0x2166a20u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x204b958; continue;
 }
 0x204b940 => {
  { let v: u32 = 0x40u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x204b94b); f_020a2012(cpu); if cpu.returned_to != 0x204b94b { return; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x204b952); f_020a20a7(cpu); if cpu.returned_to != 0x204b952 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x204b962; continue; }
  pc = 0x204b956; continue;
 }
 0x204b956 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x204b958; continue;
 }
 0x204b958 => {
  { let v: u32 = cpu.r32(0x224351cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x204b940; continue; }
  pc = 0x204b962; continue;
 }
 0x204b962 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x204b964; continue;
 }
 0x204b964 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x48u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2944(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2944); }
 let mut pc: u32 = 0x20c2944;
 loop { match pc {
 0x20c2944 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2944; continue; }
  pc = 0x20c2954; continue;
 }
 0x20c2954 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2955(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2955); }
 let mut pc: u32 = 0x20c2955;
 loop { match pc {
 0x20c2955 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2955; continue; }
  pc = 0x20c2960; continue;
 }
 0x20c2960 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2961(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2961); }
 let mut pc: u32 = 0x20c2961;
 loop { match pc {
 0x20c2961 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2961; continue; }
  pc = 0x20c297f; continue;
 }
 0x20c297f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2980); }
 let mut pc: u32 = 0x20c2980;
 loop { match pc {
 0x20c2980 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2980; continue; }
  pc = 0x20c2993; continue;
 }
 0x20c2993 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2994(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2994); }
 let mut pc: u32 = 0x20c2994;
 loop { match pc {
 0x20c2994 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2994; continue; }
  pc = 0x20c29b2; continue;
 }
 0x20c29b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c29b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c29b3); }
 let mut pc: u32 = 0x20c29b3;
 loop { match pc {
 0x20c29b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c29b3; continue; }
  pc = 0x20c29c6; continue;
 }
 0x20c29c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c29c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c29c7); }
 let mut pc: u32 = 0x20c29c7;
 loop { match pc {
 0x20c29c7 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c29c7; continue; }
  pc = 0x20c29e3; continue;
 }
 0x20c29e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c29e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c29e4); }
 let mut pc: u32 = 0x20c29e4;
 loop { match pc {
 0x20c29e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c29e4; continue; }
  pc = 0x20c29f5; continue;
 }
 0x20c29f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c29f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c29f6); }
 let mut pc: u32 = 0x20c29f6;
 loop { match pc {
 0x20c29f6 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c29f6; continue; }
  pc = 0x20c2a12; continue;
 }
 0x20c2a12 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a13); }
 let mut pc: u32 = 0x20c2a13;
 loop { match pc {
 0x20c2a13 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a13; continue; }
  pc = 0x20c2a24; continue;
 }
 0x20c2a24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a25); }
 let mut pc: u32 = 0x20c2a25;
 loop { match pc {
 0x20c2a25 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a25; continue; }
  pc = 0x20c2a38; continue;
 }
 0x20c2a38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a39); }
 let mut pc: u32 = 0x20c2a39;
 loop { match pc {
 0x20c2a39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a39; continue; }
  pc = 0x20c2a47; continue;
 }
 0x20c2a47 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a48); }
 let mut pc: u32 = 0x20c2a48;
 loop { match pc {
 0x20c2a48 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a48; continue; }
  pc = 0x20c2a5d; continue;
 }
 0x20c2a5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a5e); }
 let mut pc: u32 = 0x20c2a5e;
 loop { match pc {
 0x20c2a5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a5e; continue; }
  pc = 0x20c2a6e; continue;
 }
 0x20c2a6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a6f); }
 let mut pc: u32 = 0x20c2a6f;
 loop { match pc {
 0x20c2a6f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a6f; continue; }
  pc = 0x20c2a8f; continue;
 }
 0x20c2a8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2a90); }
 let mut pc: u32 = 0x20c2a90;
 loop { match pc {
 0x20c2a90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2a90; continue; }
  pc = 0x20c2aa5; continue;
 }
 0x20c2aa5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2aa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2aa6); }
 let mut pc: u32 = 0x20c2aa6;
 loop { match pc {
 0x20c2aa6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2aa6; continue; }
  pc = 0x20c2ac6; continue;
 }
 0x20c2ac6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2ac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2ac7); }
 let mut pc: u32 = 0x20c2ac7;
 loop { match pc {
 0x20c2ac7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2ac7; continue; }
  pc = 0x20c2adc; continue;
 }
 0x20c2adc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2add(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2add); }
 let mut pc: u32 = 0x20c2add;
 loop { match pc {
 0x20c2add => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2add; continue; }
  pc = 0x20c2b01; continue;
 }
 0x20c2b01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b02); }
 let mut pc: u32 = 0x20c2b02;
 loop { match pc {
 0x20c2b02 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2b02; continue; }
  pc = 0x20c2b1b; continue;
 }
 0x20c2b1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b1c); }
 let mut pc: u32 = 0x20c2b1c;
 loop { match pc {
 0x20c2b1c => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2b1c; continue; }
  pc = 0x20c2b40; continue;
 }
 0x20c2b40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b41); }
 let mut pc: u32 = 0x20c2b41;
 loop { match pc {
 0x20c2b41 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2b41; continue; }
  pc = 0x20c2b5a; continue;
 }
 0x20c2b5a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b5b); }
 let mut pc: u32 = 0x20c2b5b;
 loop { match pc {
 0x20c2b5b => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2b5b; continue; }
  pc = 0x20c2b7d; continue;
 }
 0x20c2b7d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b7e); }
 let mut pc: u32 = 0x20c2b7e;
 loop { match pc {
 0x20c2b7e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2b7e; continue; }
  pc = 0x20c2b95; continue;
 }
 0x20c2b95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2b96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2b96); }
 let mut pc: u32 = 0x20c2b96;
 loop { match pc {
 0x20c2b96 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2b96; continue; }
  pc = 0x20c2bb8; continue;
 }
 0x20c2bb8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2bb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2bb9); }
 let mut pc: u32 = 0x20c2bb9;
 loop { match pc {
 0x20c2bb9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2bb9; continue; }
  pc = 0x20c2bd0; continue;
 }
 0x20c2bd0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2bd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2bd1); }
 let mut pc: u32 = 0x20c2bd1;
 loop { match pc {
 0x20c2bd1 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2bd1; continue; }
  pc = 0x20c2bf7; continue;
 }
 0x20c2bf7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2bf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2bf8); }
 let mut pc: u32 = 0x20c2bf8;
 loop { match pc {
 0x20c2bf8 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2bf8; continue; }
  pc = 0x20c2c13; continue;
 }
 0x20c2c13 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2c14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2c14); }
 let mut pc: u32 = 0x20c2c14;
 loop { match pc {
 0x20c2c14 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2c14; continue; }
  pc = 0x20c2c3a; continue;
 }
 0x20c2c3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2c3b); }
 let mut pc: u32 = 0x20c2c3b;
 loop { match pc {
 0x20c2c3b => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c2c3b; continue; }
  pc = 0x20c2c56; continue;
 }
 0x20c2c56 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2c57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2c57); }
 let mut pc: u32 = 0x20c2c57;
 loop { match pc {
 0x20c2c57 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2c76; continue; }
  pc = 0x20c2c63; continue;
 }
 0x20c2c63 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2c7d; continue; }
  pc = 0x20c2c6a; continue;
 }
 0x20c2c6a => {
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2c57; continue; }
  pc = 0x20c2c75; continue;
 }
 0x20c2c75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2c76 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2c6a; continue;
 }
 0x20c2c7d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2c6a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2c84); }
 let mut pc: u32 = 0x20c2c84;
 loop { match pc {
 0x20c2c84 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2c9e; continue; }
  pc = 0x20c2c90; continue;
 }
 0x20c2c90 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2ca5; continue; }
  pc = 0x20c2c97; continue;
 }
 0x20c2c97 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2c84; continue; }
  pc = 0x20c2c9d; continue;
 }
 0x20c2c9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2c9e => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2c97; continue;
 }
 0x20c2ca5 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2c97; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2cac); }
 let mut pc: u32 = 0x20c2cac;
 loop { match pc {
 0x20c2cac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2ce9; continue; }
  pc = 0x20c2cbb; continue;
 }
 0x20c2cbb => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2cf0; continue; }
  pc = 0x20c2cc2; continue;
 }
 0x20c2cc2 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2cf7; continue; }
  pc = 0x20c2cca; continue;
 }
 0x20c2cca => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2cfe; continue; }
  pc = 0x20c2cd2; continue;
 }
 0x20c2cd2 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2cac; continue; }
  pc = 0x20c2ce8; continue;
 }
 0x20c2ce8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2ce9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2cc2; continue;
 }
 0x20c2cf0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2cc2; continue;
 }
 0x20c2cf7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2cd2; continue;
 }
 0x20c2cfe => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2cd2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2d05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2d05); }
 let mut pc: u32 = 0x20c2d05;
 loop { match pc {
 0x20c2d05 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2d37; continue; }
  pc = 0x20c2d14; continue;
 }
 0x20c2d14 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2d3e; continue; }
  pc = 0x20c2d1b; continue;
 }
 0x20c2d1b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2d45; continue; }
  pc = 0x20c2d23; continue;
 }
 0x20c2d23 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2d4c; continue; }
  pc = 0x20c2d2b; continue;
 }
 0x20c2d2b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2d05; continue; }
  pc = 0x20c2d36; continue;
 }
 0x20c2d36 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2d37 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2d1b; continue;
 }
 0x20c2d3e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2d1b; continue;
 }
 0x20c2d45 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2d2b; continue;
 }
 0x20c2d4c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2d2b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2d53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2d53); }
 let mut pc: u32 = 0x20c2d53;
 loop { match pc {
 0x20c2d53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2d90; continue; }
  pc = 0x20c2d62; continue;
 }
 0x20c2d62 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2d97; continue; }
  pc = 0x20c2d69; continue;
 }
 0x20c2d69 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2d9e; continue; }
  pc = 0x20c2d71; continue;
 }
 0x20c2d71 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2da5; continue; }
  pc = 0x20c2d79; continue;
 }
 0x20c2d79 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2d53; continue; }
  pc = 0x20c2d8f; continue;
 }
 0x20c2d8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2d90 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2d69; continue;
 }
 0x20c2d97 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2d69; continue;
 }
 0x20c2d9e => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2d79; continue;
 }
 0x20c2da5 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2d79; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2dac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2dac); }
 let mut pc: u32 = 0x20c2dac;
 loop { match pc {
 0x20c2dac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2dde; continue; }
  pc = 0x20c2dbb; continue;
 }
 0x20c2dbb => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2de5; continue; }
  pc = 0x20c2dc2; continue;
 }
 0x20c2dc2 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2dec; continue; }
  pc = 0x20c2dca; continue;
 }
 0x20c2dca => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2df3; continue; }
  pc = 0x20c2dd2; continue;
 }
 0x20c2dd2 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2dac; continue; }
  pc = 0x20c2ddd; continue;
 }
 0x20c2ddd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2dde => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2dc2; continue;
 }
 0x20c2de5 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2dc2; continue;
 }
 0x20c2dec => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2dd2; continue;
 }
 0x20c2df3 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2dd2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2dfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2dfa); }
 let mut pc: u32 = 0x20c2dfa;
 loop { match pc {
 0x20c2dfa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2e35; continue; }
  pc = 0x20c2e09; continue;
 }
 0x20c2e09 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2e3c; continue; }
  pc = 0x20c2e10; continue;
 }
 0x20c2e10 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2e43; continue; }
  pc = 0x20c2e18; continue;
 }
 0x20c2e18 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2e4a; continue; }
  pc = 0x20c2e20; continue;
 }
 0x20c2e20 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2dfa; continue; }
  pc = 0x20c2e34; continue;
 }
 0x20c2e34 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2e35 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2e10; continue;
 }
 0x20c2e3c => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2e10; continue;
 }
 0x20c2e43 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2e20; continue;
 }
 0x20c2e4a => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2e20; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2e51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2e51); }
 let mut pc: u32 = 0x20c2e51;
 loop { match pc {
 0x20c2e51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2e81; continue; }
  pc = 0x20c2e60; continue;
 }
 0x20c2e60 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2e88; continue; }
  pc = 0x20c2e67; continue;
 }
 0x20c2e67 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2e8f; continue; }
  pc = 0x20c2e6f; continue;
 }
 0x20c2e6f => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2e96; continue; }
  pc = 0x20c2e77; continue;
 }
 0x20c2e77 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2e51; continue; }
  pc = 0x20c2e80; continue;
 }
 0x20c2e80 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2e81 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2e67; continue;
 }
 0x20c2e88 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2e67; continue;
 }
 0x20c2e8f => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2e77; continue;
 }
 0x20c2e96 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2e77; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2e9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2e9d); }
 let mut pc: u32 = 0x20c2e9d;
 loop { match pc {
 0x20c2e9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2ed8; continue; }
  pc = 0x20c2eac; continue;
 }
 0x20c2eac => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2edf; continue; }
  pc = 0x20c2eb3; continue;
 }
 0x20c2eb3 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2ee6; continue; }
  pc = 0x20c2ebb; continue;
 }
 0x20c2ebb => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2eed; continue; }
  pc = 0x20c2ec3; continue;
 }
 0x20c2ec3 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2e9d; continue; }
  pc = 0x20c2ed7; continue;
 }
 0x20c2ed7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2ed8 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2eb3; continue;
 }
 0x20c2edf => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2eb3; continue;
 }
 0x20c2ee6 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2ec3; continue;
 }
 0x20c2eed => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2ec3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2ef4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2ef4); }
 let mut pc: u32 = 0x20c2ef4;
 loop { match pc {
 0x20c2ef4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2f24; continue; }
  pc = 0x20c2f03; continue;
 }
 0x20c2f03 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2f2b; continue; }
  pc = 0x20c2f0a; continue;
 }
 0x20c2f0a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2f32; continue; }
  pc = 0x20c2f12; continue;
 }
 0x20c2f12 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2f39; continue; }
  pc = 0x20c2f1a; continue;
 }
 0x20c2f1a => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2ef4; continue; }
  pc = 0x20c2f23; continue;
 }
 0x20c2f23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2f24 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f0a; continue;
 }
 0x20c2f2b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f0a; continue;
 }
 0x20c2f32 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2f1a; continue;
 }
 0x20c2f39 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c2f1a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2f40); }
 let mut pc: u32 = 0x20c2f40;
 loop { match pc {
 0x20c2f40 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2f62; continue; }
  pc = 0x20c2f4c; continue;
 }
 0x20c2f4c => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2f69; continue; }
  pc = 0x20c2f53; continue;
 }
 0x20c2f53 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2f40; continue; }
  pc = 0x20c2f61; continue;
 }
 0x20c2f61 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2f62 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f53; continue;
 }
 0x20c2f69 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f53; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2f70); }
 let mut pc: u32 = 0x20c2f70;
 loop { match pc {
 0x20c2f70 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2f8d; continue; }
  pc = 0x20c2f7c; continue;
 }
 0x20c2f7c => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2f94; continue; }
  pc = 0x20c2f83; continue;
 }
 0x20c2f83 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2f70; continue; }
  pc = 0x20c2f8c; continue;
 }
 0x20c2f8c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2f8d => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f83; continue;
 }
 0x20c2f94 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2f83; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2f9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2f9b); }
 let mut pc: u32 = 0x20c2f9b;
 loop { match pc {
 0x20c2f9b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2fbf; continue; }
  pc = 0x20c2fa7; continue;
 }
 0x20c2fa7 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2fc6; continue; }
  pc = 0x20c2fae; continue;
 }
 0x20c2fae => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2f9b; continue; }
  pc = 0x20c2fbe; continue;
 }
 0x20c2fbe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2fbf => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2fae; continue;
 }
 0x20c2fc6 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2fae; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2fcd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2fcd); }
 let mut pc: u32 = 0x20c2fcd;
 loop { match pc {
 0x20c2fcd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c2fec; continue; }
  pc = 0x20c2fd9; continue;
 }
 0x20c2fd9 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c2ff3; continue; }
  pc = 0x20c2fe0; continue;
 }
 0x20c2fe0 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2fcd; continue; }
  pc = 0x20c2feb; continue;
 }
 0x20c2feb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c2fec => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2fe0; continue;
 }
 0x20c2ff3 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c2fe0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c2ffa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c2ffa); }
 let mut pc: u32 = 0x20c2ffa;
 loop { match pc {
 0x20c2ffa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3039; continue; }
  pc = 0x20c3009; continue;
 }
 0x20c3009 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3040; continue; }
  pc = 0x20c3010; continue;
 }
 0x20c3010 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3047; continue; }
  pc = 0x20c3018; continue;
 }
 0x20c3018 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c304e; continue; }
  pc = 0x20c3020; continue;
 }
 0x20c3020 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c2ffa; continue; }
  pc = 0x20c3038; continue;
 }
 0x20c3038 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3039 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3010; continue;
 }
 0x20c3040 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3010; continue;
 }
 0x20c3047 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3020; continue;
 }
 0x20c304e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3020; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3055(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3055); }
 let mut pc: u32 = 0x20c3055;
 loop { match pc {
 0x20c3055 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3089; continue; }
  pc = 0x20c3064; continue;
 }
 0x20c3064 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3090; continue; }
  pc = 0x20c306b; continue;
 }
 0x20c306b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3097; continue; }
  pc = 0x20c3073; continue;
 }
 0x20c3073 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c309e; continue; }
  pc = 0x20c307b; continue;
 }
 0x20c307b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3055; continue; }
  pc = 0x20c3088; continue;
 }
 0x20c3088 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3089 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c306b; continue;
 }
 0x20c3090 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c306b; continue;
 }
 0x20c3097 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c307b; continue;
 }
 0x20c309e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c307b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c30a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c30a5); }
 let mut pc: u32 = 0x20c30a5;
 loop { match pc {
 0x20c30a5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c30e4; continue; }
  pc = 0x20c30b4; continue;
 }
 0x20c30b4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c30eb; continue; }
  pc = 0x20c30bb; continue;
 }
 0x20c30bb => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c30f2; continue; }
  pc = 0x20c30c3; continue;
 }
 0x20c30c3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c30f9; continue; }
  pc = 0x20c30cb; continue;
 }
 0x20c30cb => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c30a5; continue; }
  pc = 0x20c30e3; continue;
 }
 0x20c30e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c30e4 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c30bb; continue;
 }
 0x20c30eb => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c30bb; continue;
 }
 0x20c30f2 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c30cb; continue;
 }
 0x20c30f9 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c30cb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3100); }
 let mut pc: u32 = 0x20c3100;
 loop { match pc {
 0x20c3100 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3134; continue; }
  pc = 0x20c310f; continue;
 }
 0x20c310f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c313b; continue; }
  pc = 0x20c3116; continue;
 }
 0x20c3116 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3142; continue; }
  pc = 0x20c311e; continue;
 }
 0x20c311e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3149; continue; }
  pc = 0x20c3126; continue;
 }
 0x20c3126 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3100; continue; }
  pc = 0x20c3133; continue;
 }
 0x20c3133 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3134 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3116; continue;
 }
 0x20c313b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3116; continue;
 }
 0x20c3142 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3126; continue;
 }
 0x20c3149 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3126; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3150); }
 let mut pc: u32 = 0x20c3150;
 loop { match pc {
 0x20c3150 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3193; continue; }
  pc = 0x20c315f; continue;
 }
 0x20c315f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c319a; continue; }
  pc = 0x20c3166; continue;
 }
 0x20c3166 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c31a1; continue; }
  pc = 0x20c316e; continue;
 }
 0x20c316e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c31a8; continue; }
  pc = 0x20c3176; continue;
 }
 0x20c3176 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3150; continue; }
  pc = 0x20c3192; continue;
 }
 0x20c3192 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3193 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3166; continue;
 }
 0x20c319a => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3166; continue;
 }
 0x20c31a1 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3176; continue;
 }
 0x20c31a8 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3176; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c31af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c31af); }
 let mut pc: u32 = 0x20c31af;
 loop { match pc {
 0x20c31af => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c31e7; continue; }
  pc = 0x20c31be; continue;
 }
 0x20c31be => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c31ee; continue; }
  pc = 0x20c31c5; continue;
 }
 0x20c31c5 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c31f5; continue; }
  pc = 0x20c31cd; continue;
 }
 0x20c31cd => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c31fc; continue; }
  pc = 0x20c31d5; continue;
 }
 0x20c31d5 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c31af; continue; }
  pc = 0x20c31e6; continue;
 }
 0x20c31e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c31e7 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c31c5; continue;
 }
 0x20c31ee => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c31c5; continue;
 }
 0x20c31f5 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c31d5; continue;
 }
 0x20c31fc => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c31d5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3203); }
 let mut pc: u32 = 0x20c3203;
 loop { match pc {
 0x20c3203 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3246; continue; }
  pc = 0x20c3212; continue;
 }
 0x20c3212 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c324d; continue; }
  pc = 0x20c3219; continue;
 }
 0x20c3219 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3254; continue; }
  pc = 0x20c3221; continue;
 }
 0x20c3221 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c325b; continue; }
  pc = 0x20c3229; continue;
 }
 0x20c3229 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3203; continue; }
  pc = 0x20c3245; continue;
 }
 0x20c3245 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3246 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3219; continue;
 }
 0x20c324d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3219; continue;
 }
 0x20c3254 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3229; continue;
 }
 0x20c325b => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3229; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3262(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3262); }
 let mut pc: u32 = 0x20c3262;
 loop { match pc {
 0x20c3262 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c329a; continue; }
  pc = 0x20c3271; continue;
 }
 0x20c3271 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c32a1; continue; }
  pc = 0x20c3278; continue;
 }
 0x20c3278 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c32a8; continue; }
  pc = 0x20c3280; continue;
 }
 0x20c3280 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c32af; continue; }
  pc = 0x20c3288; continue;
 }
 0x20c3288 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3262; continue; }
  pc = 0x20c3299; continue;
 }
 0x20c3299 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c329a => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3278; continue;
 }
 0x20c32a1 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3278; continue;
 }
 0x20c32a8 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3288; continue;
 }
 0x20c32af => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3288; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c32b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c32b6); }
 let mut pc: u32 = 0x20c32b6;
 loop { match pc {
 0x20c32b6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c32f7; continue; }
  pc = 0x20c32c5; continue;
 }
 0x20c32c5 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c32fe; continue; }
  pc = 0x20c32cc; continue;
 }
 0x20c32cc => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3305; continue; }
  pc = 0x20c32d4; continue;
 }
 0x20c32d4 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c330c; continue; }
  pc = 0x20c32dc; continue;
 }
 0x20c32dc => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c32b6; continue; }
  pc = 0x20c32f6; continue;
 }
 0x20c32f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c32f7 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c32cc; continue;
 }
 0x20c32fe => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c32cc; continue;
 }
 0x20c3305 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c32dc; continue;
 }
 0x20c330c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c32dc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3313(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3313); }
 let mut pc: u32 = 0x20c3313;
 loop { match pc {
 0x20c3313 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3349; continue; }
  pc = 0x20c3322; continue;
 }
 0x20c3322 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3350; continue; }
  pc = 0x20c3329; continue;
 }
 0x20c3329 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3357; continue; }
  pc = 0x20c3331; continue;
 }
 0x20c3331 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c335e; continue; }
  pc = 0x20c3339; continue;
 }
 0x20c3339 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3313; continue; }
  pc = 0x20c3348; continue;
 }
 0x20c3348 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3349 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3329; continue;
 }
 0x20c3350 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3329; continue;
 }
 0x20c3357 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3339; continue;
 }
 0x20c335e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3339; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3365); }
 let mut pc: u32 = 0x20c3365;
 loop { match pc {
 0x20c3365 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c33a6; continue; }
  pc = 0x20c3374; continue;
 }
 0x20c3374 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c33ad; continue; }
  pc = 0x20c337b; continue;
 }
 0x20c337b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c33b4; continue; }
  pc = 0x20c3383; continue;
 }
 0x20c3383 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c33bb; continue; }
  pc = 0x20c338b; continue;
 }
 0x20c338b => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3365; continue; }
  pc = 0x20c33a5; continue;
 }
 0x20c33a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c33a6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c337b; continue;
 }
 0x20c33ad => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c337b; continue;
 }
 0x20c33b4 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c338b; continue;
 }
 0x20c33bb => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c338b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c33c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c33c2); }
 let mut pc: u32 = 0x20c33c2;
 loop { match pc {
 0x20c33c2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c33f8; continue; }
  pc = 0x20c33d1; continue;
 }
 0x20c33d1 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c33ff; continue; }
  pc = 0x20c33d8; continue;
 }
 0x20c33d8 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3406; continue; }
  pc = 0x20c33e0; continue;
 }
 0x20c33e0 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c340d; continue; }
  pc = 0x20c33e8; continue;
 }
 0x20c33e8 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c33c2; continue; }
  pc = 0x20c33f7; continue;
 }
 0x20c33f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c33f8 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c33d8; continue;
 }
 0x20c33ff => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c33d8; continue;
 }
 0x20c3406 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c33e8; continue;
 }
 0x20c340d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c33e8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3414(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3414); }
 let mut pc: u32 = 0x20c3414;
 loop { match pc {
 0x20c3414 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3459; continue; }
  pc = 0x20c3423; continue;
 }
 0x20c3423 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3460; continue; }
  pc = 0x20c342a; continue;
 }
 0x20c342a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3467; continue; }
  pc = 0x20c3432; continue;
 }
 0x20c3432 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c346e; continue; }
  pc = 0x20c343a; continue;
 }
 0x20c343a => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3414; continue; }
  pc = 0x20c3458; continue;
 }
 0x20c3458 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3459 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c342a; continue;
 }
 0x20c3460 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c342a; continue;
 }
 0x20c3467 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c343a; continue;
 }
 0x20c346e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c343a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3475(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3475); }
 let mut pc: u32 = 0x20c3475;
 loop { match pc {
 0x20c3475 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c34af; continue; }
  pc = 0x20c3484; continue;
 }
 0x20c3484 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c34b6; continue; }
  pc = 0x20c348b; continue;
 }
 0x20c348b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c34bd; continue; }
  pc = 0x20c3493; continue;
 }
 0x20c3493 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c34c4; continue; }
  pc = 0x20c349b; continue;
 }
 0x20c349b => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c3475; continue; }
  pc = 0x20c34ae; continue;
 }
 0x20c34ae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c34af => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c348b; continue;
 }
 0x20c34b6 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c348b; continue;
 }
 0x20c34bd => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c349b; continue;
 }
 0x20c34c4 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c349b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c34cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c34cb); }
 let mut pc: u32 = 0x20c34cb;
 loop { match pc {
 0x20c34cb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3510; continue; }
  pc = 0x20c34da; continue;
 }
 0x20c34da => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3517; continue; }
  pc = 0x20c34e1; continue;
 }
 0x20c34e1 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c351e; continue; }
  pc = 0x20c34e9; continue;
 }
 0x20c34e9 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c3525; continue; }
  pc = 0x20c34f1; continue;
 }
 0x20c34f1 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c34cb; continue; }
  pc = 0x20c350f; continue;
 }
 0x20c350f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3510 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c34e1; continue;
 }
 0x20c3517 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c34e1; continue;
 }
 0x20c351e => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c34f1; continue;
 }
 0x20c3525 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c34f1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c352c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c352c); }
 let mut pc: u32 = 0x20c352c;
 loop { match pc {
 0x20c352c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3566; continue; }
  pc = 0x20c353b; continue;
 }
 0x20c353b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c356d; continue; }
  pc = 0x20c3542; continue;
 }
 0x20c3542 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c3574; continue; }
  pc = 0x20c354a; continue;
 }
 0x20c354a => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c357b; continue; }
  pc = 0x20c3552; continue;
 }
 0x20c3552 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c352c; continue; }
  pc = 0x20c3565; continue;
 }
 0x20c3565 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c3566 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3542; continue;
 }
 0x20c356d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c3542; continue;
 }
 0x20c3574 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3552; continue;
 }
 0x20c357b => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x20c3552; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3582(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3582); }
 let mut pc: u32 = 0x20c3582;
 loop { match pc {
 0x20c3582 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c35a2; continue; }
  pc = 0x20c358a; continue;
 }
 0x20c358a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3582; continue; }
  pc = 0x20c35a2; continue;
 }
 0x20c35a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c35a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c35a3); }
 let mut pc: u32 = 0x20c35a3;
 loop { match pc {
 0x20c35a3 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c35c6; continue; }
  pc = 0x20c35ab; continue;
 }
 0x20c35ab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c35a3; continue; }
  pc = 0x20c35c6; continue;
 }
 0x20c35c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c35c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c35c7); }
 let mut pc: u32 = 0x20c35c7;
 loop { match pc {
 0x20c35c7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c35f8; continue; }
  pc = 0x20c35cf; continue;
 }
 0x20c35cf => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c35c7; continue; }
  pc = 0x20c35f8; continue;
 }
 0x20c35f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c35f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c35f9); }
 let mut pc: u32 = 0x20c35f9;
 loop { match pc {
 0x20c35f9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c362a; continue; }
  pc = 0x20c3601; continue;
 }
 0x20c3601 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c35f9; continue; }
  pc = 0x20c362a; continue;
 }
 0x20c362a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c362b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c362b); }
 let mut pc: u32 = 0x20c362b;
 loop { match pc {
 0x20c362b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c364e; continue; }
  pc = 0x20c3633; continue;
 }
 0x20c3633 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c362b; continue; }
  pc = 0x20c364e; continue;
 }
 0x20c364e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c364f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c364f); }
 let mut pc: u32 = 0x20c364f;
 loop { match pc {
 0x20c364f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3675; continue; }
  pc = 0x20c3657; continue;
 }
 0x20c3657 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c364f; continue; }
  pc = 0x20c3675; continue;
 }
 0x20c3675 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3676(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3676); }
 let mut pc: u32 = 0x20c3676;
 loop { match pc {
 0x20c3676 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c36a9; continue; }
  pc = 0x20c367e; continue;
 }
 0x20c367e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3676; continue; }
  pc = 0x20c36a9; continue;
 }
 0x20c36a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c36aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c36aa); }
 let mut pc: u32 = 0x20c36aa;
 loop { match pc {
 0x20c36aa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c36dd; continue; }
  pc = 0x20c36b2; continue;
 }
 0x20c36b2 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c36aa; continue; }
  pc = 0x20c36dd; continue;
 }
 0x20c36dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c36de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c36de); }
 let mut pc: u32 = 0x20c36de;
 loop { match pc {
 0x20c36de => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c36f9; continue; }
  pc = 0x20c36e6; continue;
 }
 0x20c36e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c36de; continue; }
  pc = 0x20c36f9; continue;
 }
 0x20c36f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c36fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c36fa); }
 let mut pc: u32 = 0x20c36fa;
 loop { match pc {
 0x20c36fa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3718; continue; }
  pc = 0x20c3702; continue;
 }
 0x20c3702 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c36fa; continue; }
  pc = 0x20c3718; continue;
 }
 0x20c3718 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3719(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3719); }
 let mut pc: u32 = 0x20c3719;
 loop { match pc {
 0x20c3719 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c373f; continue; }
  pc = 0x20c3721; continue;
 }
 0x20c3721 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3719; continue; }
  pc = 0x20c373f; continue;
 }
 0x20c373f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3740); }
 let mut pc: u32 = 0x20c3740;
 loop { match pc {
 0x20c3740 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3766; continue; }
  pc = 0x20c3748; continue;
 }
 0x20c3748 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3740; continue; }
  pc = 0x20c3766; continue;
 }
 0x20c3766 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3767(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3767); }
 let mut pc: u32 = 0x20c3767;
 loop { match pc {
 0x20c3767 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3785; continue; }
  pc = 0x20c376f; continue;
 }
 0x20c376f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3767; continue; }
  pc = 0x20c3785; continue;
 }
 0x20c3785 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3786(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3786); }
 let mut pc: u32 = 0x20c3786;
 loop { match pc {
 0x20c3786 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c37a7; continue; }
  pc = 0x20c378e; continue;
 }
 0x20c378e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3786; continue; }
  pc = 0x20c37a7; continue;
 }
 0x20c37a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c37a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c37a8); }
 let mut pc: u32 = 0x20c37a8;
 loop { match pc {
 0x20c37a8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c37d0; continue; }
  pc = 0x20c37b0; continue;
 }
 0x20c37b0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c37a8; continue; }
  pc = 0x20c37d0; continue;
 }
 0x20c37d0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c37d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c37d1); }
 let mut pc: u32 = 0x20c37d1;
 loop { match pc {
 0x20c37d1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c37f9; continue; }
  pc = 0x20c37d9; continue;
 }
 0x20c37d9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c37d1; continue; }
  pc = 0x20c37f9; continue;
 }
 0x20c37f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c37fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c37fa); }
 let mut pc: u32 = 0x20c37fa;
 loop { match pc {
 0x20c37fa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3825; continue; }
  pc = 0x20c3802; continue;
 }
 0x20c3802 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c37fa; continue; }
  pc = 0x20c3825; continue;
 }
 0x20c3825 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3826(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3826); }
 let mut pc: u32 = 0x20c3826;
 loop { match pc {
 0x20c3826 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3854; continue; }
  pc = 0x20c382e; continue;
 }
 0x20c382e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3826; continue; }
  pc = 0x20c3854; continue;
 }
 0x20c3854 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3855); }
 let mut pc: u32 = 0x20c3855;
 loop { match pc {
 0x20c3855 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3894; continue; }
  pc = 0x20c385d; continue;
 }
 0x20c385d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3855; continue; }
  pc = 0x20c3894; continue;
 }
 0x20c3894 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3895(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3895); }
 let mut pc: u32 = 0x20c3895;
 loop { match pc {
 0x20c3895 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c38d4; continue; }
  pc = 0x20c389d; continue;
 }
 0x20c389d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3895; continue; }
  pc = 0x20c38d4; continue;
 }
 0x20c38d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c38d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c38d5); }
 let mut pc: u32 = 0x20c38d5;
 loop { match pc {
 0x20c38d5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3906; continue; }
  pc = 0x20c38dd; continue;
 }
 0x20c38dd => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c38d5; continue; }
  pc = 0x20c3906; continue;
 }
 0x20c3906 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3907(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3907); }
 let mut pc: u32 = 0x20c3907;
 loop { match pc {
 0x20c3907 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c393b; continue; }
  pc = 0x20c390f; continue;
 }
 0x20c390f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3907; continue; }
  pc = 0x20c393b; continue;
 }
 0x20c393b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c393c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c393c); }
 let mut pc: u32 = 0x20c393c;
 loop { match pc {
 0x20c393c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c397d; continue; }
  pc = 0x20c3944; continue;
 }
 0x20c3944 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c393c; continue; }
  pc = 0x20c397d; continue;
 }
 0x20c397d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c397e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c397e); }
 let mut pc: u32 = 0x20c397e;
 loop { match pc {
 0x20c397e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c39bf; continue; }
  pc = 0x20c3986; continue;
 }
 0x20c3986 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c397e; continue; }
  pc = 0x20c39bf; continue;
 }
 0x20c39bf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c39c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c39c0); }
 let mut pc: u32 = 0x20c39c0;
 loop { match pc {
 0x20c39c0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c39e6; continue; }
  pc = 0x20c39c8; continue;
 }
 0x20c39c8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c39c0; continue; }
  pc = 0x20c39e6; continue;
 }
 0x20c39e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c39e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c39e7); }
 let mut pc: u32 = 0x20c39e7;
 loop { match pc {
 0x20c39e7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3a10; continue; }
  pc = 0x20c39ef; continue;
 }
 0x20c39ef => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c39e7; continue; }
  pc = 0x20c3a10; continue;
 }
 0x20c3a10 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3a11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3a11); }
 let mut pc: u32 = 0x20c3a11;
 loop { match pc {
 0x20c3a11 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3a45; continue; }
  pc = 0x20c3a19; continue;
 }
 0x20c3a19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3a11; continue; }
  pc = 0x20c3a45; continue;
 }
 0x20c3a45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3a46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3a46); }
 let mut pc: u32 = 0x20c3a46;
 loop { match pc {
 0x20c3a46 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3a7a; continue; }
  pc = 0x20c3a4e; continue;
 }
 0x20c3a4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3a46; continue; }
  pc = 0x20c3a7a; continue;
 }
 0x20c3a7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3a7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3a7b); }
 let mut pc: u32 = 0x20c3a7b;
 loop { match pc {
 0x20c3a7b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3aa7; continue; }
  pc = 0x20c3a83; continue;
 }
 0x20c3a83 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3a7b; continue; }
  pc = 0x20c3aa7; continue;
 }
 0x20c3aa7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3aa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3aa8); }
 let mut pc: u32 = 0x20c3aa8;
 loop { match pc {
 0x20c3aa8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3ad7; continue; }
  pc = 0x20c3ab0; continue;
 }
 0x20c3ab0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3aa8; continue; }
  pc = 0x20c3ad7; continue;
 }
 0x20c3ad7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3ad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3ad8); }
 let mut pc: u32 = 0x20c3ad8;
 loop { match pc {
 0x20c3ad8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3b0e; continue; }
  pc = 0x20c3ae0; continue;
 }
 0x20c3ae0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3ad8; continue; }
  pc = 0x20c3b0e; continue;
 }
 0x20c3b0e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3b0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3b0f); }
 let mut pc: u32 = 0x20c3b0f;
 loop { match pc {
 0x20c3b0f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3b45; continue; }
  pc = 0x20c3b17; continue;
 }
 0x20c3b17 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3b0f; continue; }
  pc = 0x20c3b45; continue;
 }
 0x20c3b45 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3b46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3b46); }
 let mut pc: u32 = 0x20c3b46;
 loop { match pc {
 0x20c3b46 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3b61; continue; }
  pc = 0x20c3b4e; continue;
 }
 0x20c3b4e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3b46; continue; }
  pc = 0x20c3b61; continue;
 }
 0x20c3b61 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3b62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3b62); }
 let mut pc: u32 = 0x20c3b62;
 loop { match pc {
 0x20c3b62 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3b87; continue; }
  pc = 0x20c3b6a; continue;
 }
 0x20c3b6a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3b62; continue; }
  pc = 0x20c3b87; continue;
 }
 0x20c3b87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3b88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3b88); }
 let mut pc: u32 = 0x20c3b88;
 loop { match pc {
 0x20c3b88 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3bb1; continue; }
  pc = 0x20c3b90; continue;
 }
 0x20c3b90 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3b88; continue; }
  pc = 0x20c3bb1; continue;
 }
 0x20c3bb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3bb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3bb2); }
 let mut pc: u32 = 0x20c3bb2;
 loop { match pc {
 0x20c3bb2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3bdc; continue; }
  pc = 0x20c3bba; continue;
 }
 0x20c3bba => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3bb2; continue; }
  pc = 0x20c3bdc; continue;
 }
 0x20c3bdc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3bdd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3bdd); }
 let mut pc: u32 = 0x20c3bdd;
 loop { match pc {
 0x20c3bdd => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3c06; continue; }
  pc = 0x20c3be5; continue;
 }
 0x20c3be5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3bdd; continue; }
  pc = 0x20c3c06; continue;
 }
 0x20c3c06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3c07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3c07); }
 let mut pc: u32 = 0x20c3c07;
 loop { match pc {
 0x20c3c07 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3c31; continue; }
  pc = 0x20c3c0f; continue;
 }
 0x20c3c0f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3c07; continue; }
  pc = 0x20c3c31; continue;
 }
 0x20c3c31 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3c32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3c32); }
 let mut pc: u32 = 0x20c3c32;
 loop { match pc {
 0x20c3c32 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3c5b; continue; }
  pc = 0x20c3c3a; continue;
 }
 0x20c3c3a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3c32; continue; }
  pc = 0x20c3c5b; continue;
 }
 0x20c3c5b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3c5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3c5c); }
 let mut pc: u32 = 0x20c3c5c;
 loop { match pc {
 0x20c3c5c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3c95; continue; }
  pc = 0x20c3c64; continue;
 }
 0x20c3c64 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3c5c; continue; }
  pc = 0x20c3c95; continue;
 }
 0x20c3c95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3c96); }
 let mut pc: u32 = 0x20c3c96;
 loop { match pc {
 0x20c3c96 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3cd8; continue; }
  pc = 0x20c3c9e; continue;
 }
 0x20c3c9e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3c96; continue; }
  pc = 0x20c3cd8; continue;
 }
 0x20c3cd8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3cd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3cd9); }
 let mut pc: u32 = 0x20c3cd9;
 loop { match pc {
 0x20c3cd9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3d1c; continue; }
  pc = 0x20c3ce1; continue;
 }
 0x20c3ce1 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3cd9; continue; }
  pc = 0x20c3d1c; continue;
 }
 0x20c3d1c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3d1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3d1d); }
 let mut pc: u32 = 0x20c3d1d;
 loop { match pc {
 0x20c3d1d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3d5f; continue; }
  pc = 0x20c3d25; continue;
 }
 0x20c3d25 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3d1d; continue; }
  pc = 0x20c3d5f; continue;
 }
 0x20c3d5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3d60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3d60); }
 let mut pc: u32 = 0x20c3d60;
 loop { match pc {
 0x20c3d60 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3da3; continue; }
  pc = 0x20c3d68; continue;
 }
 0x20c3d68 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3d60; continue; }
  pc = 0x20c3da3; continue;
 }
 0x20c3da3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3da4); }
 let mut pc: u32 = 0x20c3da4;
 loop { match pc {
 0x20c3da4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3dc8; continue; }
  pc = 0x20c3dac; continue;
 }
 0x20c3dac => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3da4; continue; }
  pc = 0x20c3dc8; continue;
 }
 0x20c3dc8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3dc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3dc9); }
 let mut pc: u32 = 0x20c3dc9;
 loop { match pc {
 0x20c3dc9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3dfd; continue; }
  pc = 0x20c3dd1; continue;
 }
 0x20c3dd1 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3dc9; continue; }
  pc = 0x20c3dfd; continue;
 }
 0x20c3dfd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3dfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3dfe); }
 let mut pc: u32 = 0x20c3dfe;
 loop { match pc {
 0x20c3dfe => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3e35; continue; }
  pc = 0x20c3e06; continue;
 }
 0x20c3e06 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3dfe; continue; }
  pc = 0x20c3e35; continue;
 }
 0x20c3e35 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3e36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3e36); }
 let mut pc: u32 = 0x20c3e36;
 loop { match pc {
 0x20c3e36 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3e6e; continue; }
  pc = 0x20c3e3e; continue;
 }
 0x20c3e3e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3e36; continue; }
  pc = 0x20c3e6e; continue;
 }
 0x20c3e6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3e6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3e6f); }
 let mut pc: u32 = 0x20c3e6f;
 loop { match pc {
 0x20c3e6f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3ea6; continue; }
  pc = 0x20c3e77; continue;
 }
 0x20c3e77 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3e6f; continue; }
  pc = 0x20c3ea6; continue;
 }
 0x20c3ea6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3ea7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3ea7); }
 let mut pc: u32 = 0x20c3ea7;
 loop { match pc {
 0x20c3ea7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3edf; continue; }
  pc = 0x20c3eaf; continue;
 }
 0x20c3eaf => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3ea7; continue; }
  pc = 0x20c3edf; continue;
 }
 0x20c3edf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3ee0); }
 let mut pc: u32 = 0x20c3ee0;
 loop { match pc {
 0x20c3ee0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3f06; continue; }
  pc = 0x20c3ee8; continue;
 }
 0x20c3ee8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3ee0; continue; }
  pc = 0x20c3f06; continue;
 }
 0x20c3f06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3f07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3f07); }
 let mut pc: u32 = 0x20c3f07;
 loop { match pc {
 0x20c3f07 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3f37; continue; }
  pc = 0x20c3f0f; continue;
 }
 0x20c3f0f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x217cb68u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3f07; continue; }
  pc = 0x20c3f37; continue;
 }
 0x20c3f37 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3f38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3f38); }
 let mut pc: u32 = 0x20c3f38;
 loop { match pc {
 0x20c3f38 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3f6f; continue; }
  pc = 0x20c3f40; continue;
 }
 0x20c3f40 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3f38; continue; }
  pc = 0x20c3f6f; continue;
 }
 0x20c3f6f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3f70); }
 let mut pc: u32 = 0x20c3f70;
 loop { match pc {
 0x20c3f70 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3fa8; continue; }
  pc = 0x20c3f78; continue;
 }
 0x20c3f78 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3f70; continue; }
  pc = 0x20c3fa8; continue;
 }
 0x20c3fa8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3fa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3fa9); }
 let mut pc: u32 = 0x20c3fa9;
 loop { match pc {
 0x20c3fa9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c3fe0; continue; }
  pc = 0x20c3fb1; continue;
 }
 0x20c3fb1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3fa9; continue; }
  pc = 0x20c3fe0; continue;
 }
 0x20c3fe0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c3fe1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c3fe1); }
 let mut pc: u32 = 0x20c3fe1;
 loop { match pc {
 0x20c3fe1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4019; continue; }
  pc = 0x20c3fe9; continue;
 }
 0x20c3fe9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c3fe1; continue; }
  pc = 0x20c4019; continue;
 }
 0x20c4019 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c401a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c401a); }
 let mut pc: u32 = 0x20c401a;
 loop { match pc {
 0x20c401a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4051; continue; }
  pc = 0x20c4022; continue;
 }
 0x20c4022 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c401a; continue; }
  pc = 0x20c4051; continue;
 }
 0x20c4051 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4052(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4052); }
 let mut pc: u32 = 0x20c4052;
 loop { match pc {
 0x20c4052 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4099; continue; }
  pc = 0x20c405a; continue;
 }
 0x20c405a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c4052; continue; }
  pc = 0x20c4099; continue;
 }
 0x20c4099 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c40eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c40eb); }
 let mut pc: u32 = 0x20c40eb;
 loop { match pc {
 0x20c40eb => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c413c; continue; }
  pc = 0x20c40f3; continue;
 }
 0x20c40f3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c40eb; continue; }
  pc = 0x20c413c; continue;
 }
 0x20c413c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c413d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c413d); }
 let mut pc: u32 = 0x20c413d;
 loop { match pc {
 0x20c413d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c418d; continue; }
  pc = 0x20c4145; continue;
 }
 0x20c4145 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c413d; continue; }
  pc = 0x20c418d; continue;
 }
 0x20c418d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c418e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c418e); }
 let mut pc: u32 = 0x20c418e;
 loop { match pc {
 0x20c418e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c41df; continue; }
  pc = 0x20c4196; continue;
 }
 0x20c4196 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c418e; continue; }
  pc = 0x20c41df; continue;
 }
 0x20c41df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c41e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c41e0); }
 let mut pc: u32 = 0x20c41e0;
 loop { match pc {
 0x20c41e0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4212; continue; }
  pc = 0x20c41e8; continue;
 }
 0x20c41e8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c41e0; continue; }
  pc = 0x20c4212; continue;
 }
 0x20c4212 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4213); }
 let mut pc: u32 = 0x20c4213;
 loop { match pc {
 0x20c4213 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4255; continue; }
  pc = 0x20c421b; continue;
 }
 0x20c421b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c4213; continue; }
  pc = 0x20c4255; continue;
 }
 0x20c4255 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4256); }
 let mut pc: u32 = 0x20c4256;
 loop { match pc {
 0x20c4256 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c429b; continue; }
  pc = 0x20c425e; continue;
 }
 0x20c425e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c4256; continue; }
  pc = 0x20c429b; continue;
 }
 0x20c429b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c429c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c429c); }
 let mut pc: u32 = 0x20c429c;
 loop { match pc {
 0x20c429c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c42e2; continue; }
  pc = 0x20c42a4; continue;
 }
 0x20c42a4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c429c; continue; }
  pc = 0x20c42e2; continue;
 }
 0x20c42e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c42e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c42e3); }
 let mut pc: u32 = 0x20c42e3;
 loop { match pc {
 0x20c42e3 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c4328; continue; }
  pc = 0x20c42eb; continue;
 }
 0x20c42eb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c42e3; continue; }
  pc = 0x20c4328; continue;
 }
 0x20c4328 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4329(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4329); }
 let mut pc: u32 = 0x20c4329;
 loop { match pc {
 0x20c4329 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c436f; continue; }
  pc = 0x20c4331; continue;
 }
 0x20c4331 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c4329; continue; }
  pc = 0x20c436f; continue;
 }
 0x20c436f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
