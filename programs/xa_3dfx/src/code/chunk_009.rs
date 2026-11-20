#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_01110c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1110c24); }
 let mut pc: u32 = 0x1110c24;
 loop { match pc {
 0x1110c24 => {
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8) | 0x11u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x11u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), cpu.edx); cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.call(0x1110c34); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0111158c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x111158c); }
 let mut pc: u32 = 0x111158c;
 loop { match pc {
 0x111158c => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x11115a0; continue; }
  pc = 0x111159d; continue;
 }
 0x111159d => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x11115a0; continue;
 }
 0x11115a0 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { pc = 0x11115a8; continue; }
  pc = 0x11115a6; continue;
 }
 0x11115a6 => {
  { cpu.stos(2, false); }
  pc = 0x11115a8; continue;
 }
 0x11115a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_011115a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11115a9); }
 let mut pc: u32 = 0x11115a9;
 loop { match pc {
 0x11115a9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x11115b0; continue;
 }
 0x11115b0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x11115b2; continue;
 }
 0x11115b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11115b2; continue; }
  pc = 0x11115be; continue;
 }
 0x11115be => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11115b0; continue; }
  pc = 0x11115c3; continue;
 }
 0x11115c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_011117df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11117df); }
 let mut pc: u32 = 0x11117df;
 loop { match pc {
 0x11117df => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x1111817; continue; }
  pc = 0x11117f5; continue;
 }
 0x11117f5 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x1111807; continue; }
  pc = 0x11117f9; continue;
 }
 0x11117f9 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11117f9; continue; }
  pc = 0x1111804; continue;
 }
 0x1111804 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1111807 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1111807; continue; }
  pc = 0x1111814; continue;
 }
 0x1111814 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x1111817 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x111182e; continue; }
  pc = 0x111181b; continue;
 }
 0x111181b => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x111181c; continue;
 }
 0x111181c => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x111181c; continue; }
  pc = 0x111182b; continue;
 }
 0x111182b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x111182e => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x111182e; continue; }
  pc = 0x111183b; continue;
 }
 0x111183b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0111183e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x111183e); }
 let mut pc: u32 = 0x111183e;
 loop { match pc {
 0x111183e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x12f1eb0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x12f1eacu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x1111a55; continue;
 }
 0x1111869 => {
  { let v: u32 = cpu.r32(0x12f1eacu32); cpu.ecx = v; }
  pc = 0x1111938; continue;
 }
 0x1111874 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x111187e; continue; }
  pc = 0x111187b; continue;
 }
 0x111187b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x111187e; continue;
 }
 0x111187e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x111188a; continue; }
  pc = 0x1111886; continue;
 }
 0x1111886 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x111188a; continue;
 }
 0x111188a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111896; continue; }
  pc = 0x1111892; continue;
 }
 0x1111892 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111896; continue;
 }
 0x1111896 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118a2; continue; }
  pc = 0x111189e; continue;
 }
 0x111189e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x11118a2; continue;
 }
 0x11118a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118ae; continue; }
  pc = 0x11118aa; continue;
 }
 0x11118aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x11118ae; continue;
 }
 0x11118ae => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118ba; continue; }
  pc = 0x11118b6; continue;
 }
 0x11118b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x11118ba; continue;
 }
 0x11118ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118c6; continue; }
  pc = 0x11118c2; continue;
 }
 0x11118c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x11118c6; continue;
 }
 0x11118c6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118d2; continue; }
  pc = 0x11118ce; continue;
 }
 0x11118ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x11118d2; continue;
 }
 0x11118d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118de; continue; }
  pc = 0x11118da; continue;
 }
 0x11118da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x11118de; continue;
 }
 0x11118de => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118ea; continue; }
  pc = 0x11118e6; continue;
 }
 0x11118e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x11118ea; continue;
 }
 0x11118ea => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11118f6; continue; }
  pc = 0x11118f2; continue;
 }
 0x11118f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x11118f6; continue;
 }
 0x11118f6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111902; continue; }
  pc = 0x11118fe; continue;
 }
 0x11118fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111902; continue;
 }
 0x1111902 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x111190e; continue; }
  pc = 0x111190a; continue;
 }
 0x111190a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x111190e; continue;
 }
 0x111190e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x111191a; continue; }
  pc = 0x1111916; continue;
 }
 0x1111916 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x111191a; continue;
 }
 0x111191a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111926; continue; }
  pc = 0x1111922; continue;
 }
 0x1111922 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x1111926; continue;
 }
 0x1111926 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111932; continue; }
  pc = 0x111192e; continue;
 }
 0x111192e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x1111932; continue;
 }
 0x1111932 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x1111938; continue;
 }
 0x1111938 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1111874; continue; }
  pc = 0x1111941; continue;
 }
 0x1111941 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x111194d; continue;
 }
 0x111194d => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111956; continue; }
  pc = 0x1111953; continue;
 }
 0x1111953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111956; continue;
 }
 0x1111956 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111962; continue; }
  pc = 0x111195e; continue;
 }
 0x111195e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111962; continue;
 }
 0x1111962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x111196e; continue;
 }
 0x111196e => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111977; continue; }
  pc = 0x1111974; continue;
 }
 0x1111974 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111977; continue;
 }
 0x1111977 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111983; continue; }
  pc = 0x111197f; continue;
 }
 0x111197f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111983; continue;
 }
 0x1111983 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x111198f; continue;
 }
 0x111198f => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111998; continue; }
  pc = 0x1111995; continue;
 }
 0x1111995 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111998; continue;
 }
 0x1111998 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119a4; continue; }
  pc = 0x11119a0; continue;
 }
 0x11119a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119a4; continue;
 }
 0x11119a4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x11119b0; continue;
 }
 0x11119b0 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119b9; continue; }
  pc = 0x11119b6; continue;
 }
 0x11119b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119b9; continue;
 }
 0x11119b9 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119c5; continue; }
  pc = 0x11119c1; continue;
 }
 0x11119c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119c5; continue;
 }
 0x11119c5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x11119cd; continue;
 }
 0x11119cd => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119d6; continue; }
  pc = 0x11119d3; continue;
 }
 0x11119d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119d6; continue;
 }
 0x11119d6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119e2; continue; }
  pc = 0x11119de; continue;
 }
 0x11119de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119e2; continue;
 }
 0x11119e2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x11119ea; continue;
 }
 0x11119ea => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119f3; continue; }
  pc = 0x11119f0; continue;
 }
 0x11119f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119f3; continue;
 }
 0x11119f3 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x11119ff; continue; }
  pc = 0x11119fb; continue;
 }
 0x11119fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x11119ff; continue;
 }
 0x11119ff => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x1111a07; continue;
 }
 0x1111a07 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111a10; continue; }
  pc = 0x1111a0d; continue;
 }
 0x1111a0d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111a10; continue;
 }
 0x1111a10 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111a1c; continue; }
  pc = 0x1111a18; continue;
 }
 0x1111a18 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111a1c; continue;
 }
 0x1111a1c => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x1111a3c; continue; }
  pc = 0x1111a24; continue;
 }
 0x1111a24 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111a2d; continue; }
  pc = 0x1111a2a; continue;
 }
 0x1111a2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111a2d; continue;
 }
 0x1111a2d => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111a39; continue; }
  pc = 0x1111a35; continue;
 }
 0x1111a35 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111a39; continue;
 }
 0x1111a39 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x1111a3c; continue;
 }
 0x1111a3c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x1111a51; continue; }
  pc = 0x1111a44; continue;
 }
 0x1111a44 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x1111a4e; continue; }
  pc = 0x1111a4b; continue;
 }
 0x1111a4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x1111a4e; continue;
 }
 0x1111a4e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x1111a51; continue;
 }
 0x1111a51 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x1111a55; continue;
 }
 0x1111a55 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x12f1eb0u32)); cpu.w32(0x12f1eb0u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x1111869; continue; }
  pc = 0x1111a61; continue;
 }
 0x1111a61 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01111a65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1111a65); }
 let mut pc: u32 = 0x1111a65;
 loop { match pc {
 0x1111a65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x12f1eb8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12f1eb4u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x12f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x1111a84; continue;
 }
 0x1111a84 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.sf { pc = 0x1111a93; continue; }
  pc = 0x1111a88; continue;
 }
 0x1111a88 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x12f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  pc = 0x1111a93; continue;
 }
 0x1111a93 => {
  { let v: u32 = cpu.r32(0x12f1eb8u32); cpu.edi = v; }
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x12f1eb4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x1111a84; continue; }
  pc = 0x1111adf; continue;
 }
 0x1111adf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01111ae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1111ae1); }
 let mut pc: u32 = 0x1111ae1;
 loop { match pc {
 0x1111ae1 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1111ae1; continue; }
  pc = 0x1111aeb; continue;
 }
 0x1111aeb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01120000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1120000); } r_01120000(cpu, 0x1120000); }
pub fn f_011200f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x11200f4); } r_01120000(cpu, 0x11200f4); }
fn r_01120000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1120000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1251b4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1251b48u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112018a; continue; }
  pc = 0x1120023; continue;
 }
 0x1120023 => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112006d; continue; }
  pc = 0x112002f; continue;
 }
 0x112002f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120064; continue; }
  pc = 0x1120053; continue;
 }
 0x1120053 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120064; continue;
 }
 0x1120064 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112006d; continue;
 }
 0x112006d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x11200a2; continue;
 }
 0x1120072 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11200a2; continue;
 }
 0x11200a2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ecx = v; }
  pc = 0x11200ab; continue;
 }
 0x11200ab => {
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
  if !cpu.zf { cpu.poll(); pc = 0x11200ab; continue; }
  pc = 0x11200ce; continue;
 }
 0x11200ce => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x1120072; continue; }
  pc = 0x11200d8; continue;
 }
 0x11200d8 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x1120114; continue;
 }
 0x11200f4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0xffffffb8u32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffffb8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1bb664fu32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0xffffff8bu32.wrapping_add(cpu.esi)), ((cpu.eax >> 8) as u8)); cpu.w8(0xffffff8bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x1251bbau32); cpu.eax = v; }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x1120114; continue;
 }
 0x1120114 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x1120114; continue; }
  pc = 0x1120137; continue;
 }
 0x1120137 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120185; continue; }
  pc = 0x1120147; continue;
 }
 0x1120147 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112017c; continue; }
  pc = 0x112016b; continue;
 }
 0x112016b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112017c; continue;
 }
 0x112017c => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120185; continue;
 }
 0x1120185 => {
  pc = 0x112027a; continue;
 }
 0x112018a => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11201d4; continue; }
  pc = 0x1120196; continue;
 }
 0x1120196 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11201cb; continue; }
  pc = 0x11201ba; continue;
 }
 0x11201ba => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11201cb; continue;
 }
 0x11201cb => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11201d4; continue;
 }
 0x11201d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x11201f7; continue;
 }
 0x11201d9 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { cpu.interrupt(0x10); }
  pc = 0x11201f7; continue;
 }
 0x11201f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ecx = v; }
  pc = 0x1120200; continue;
 }
 0x1120200 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x1120200; continue; }
  pc = 0x1120223; continue;
 }
 0x1120223 => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x11201d9; continue; }
  pc = 0x112022d; continue;
 }
 0x112022d => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x1120257; continue;
 }
 0x1120257 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x1120257; continue; }
  pc = 0x112027a; continue;
 }
 0x112027a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112027c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112027c); }
 let mut pc: u32 = 0x112027c;
 loop { match pc {
 0x112027c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11202f1; continue; }
  pc = 0x11202b3; continue;
 }
 0x11202b3 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11202e8; continue; }
  pc = 0x11202d7; continue;
 }
 0x11202d7 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11202e8; continue;
 }
 0x11202e8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11202f1; continue;
 }
 0x11202f1 => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x112032f; continue;
 }
 0x112032f => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x1120350; continue; }
  pc = 0x1120337; continue;
 }
 0x1120337 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112032f; continue; }
  pc = 0x112034b; continue;
 }
 0x112034b => {
  pc = 0x1120474; continue;
 }
 0x1120350 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11203fb; continue; }
  pc = 0x1120361; continue;
 }
 0x1120361 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11203c6; continue; }
  pc = 0x1120388; continue;
 }
 0x1120388 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11203bd; continue; }
  pc = 0x11203ac; continue;
 }
 0x11203ac => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11203bd; continue;
 }
 0x11203bd => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11203c6; continue;
 }
 0x11203c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112032f; continue; }
  pc = 0x11203f9; continue;
 }
 0x11203f9 => {
  pc = 0x1120474; continue;
 }
 0x11203fb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112045d; continue; }
  pc = 0x112041f; continue;
 }
 0x112041f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120454; continue; }
  pc = 0x1120443; continue;
 }
 0x1120443 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120454; continue;
 }
 0x1120454 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112045d; continue;
 }
 0x112045d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112032f; continue; }
  pc = 0x1120474; continue;
 }
 0x1120474 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01120477(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1120477); } r_01120477(cpu, 0x1120477); }
pub fn f_0112048d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x112048d); } r_01120477(cpu, 0x112048d); }
fn r_01120477(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1120477 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  pc = 0x112048d; continue;
 }
 0x112048d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11204ec; continue; }
  pc = 0x11204ae; continue;
 }
 0x11204ae => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11204e3; continue; }
  pc = 0x11204d2; continue;
 }
 0x11204d2 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11204e3; continue;
 }
 0x11204e3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11204ec; continue;
 }
 0x11204ec => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x112052a; continue;
 }
 0x112052a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x112054b; continue; }
  pc = 0x1120532; continue;
 }
 0x1120532 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112052a; continue; }
  pc = 0x1120546; continue;
 }
 0x1120546 => {
  pc = 0x1120671; continue;
 }
 0x112054b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11205f8; continue; }
  pc = 0x112055c; continue;
 }
 0x112055c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11205c1; continue; }
  pc = 0x1120583; continue;
 }
 0x1120583 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11205b8; continue; }
  pc = 0x11205a7; continue;
 }
 0x11205a7 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11205b8; continue;
 }
 0x11205b8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11205c1; continue;
 }
 0x11205c1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112052a; continue; }
  pc = 0x11205f6; continue;
 }
 0x11205f6 => {
  pc = 0x1120671; continue;
 }
 0x11205f8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112065a; continue; }
  pc = 0x112061c; continue;
 }
 0x112061c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120651; continue; }
  pc = 0x1120640; continue;
 }
 0x1120640 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120651; continue;
 }
 0x1120651 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112065a; continue;
 }
 0x112065a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112052a; continue; }
  pc = 0x1120671; continue;
 }
 0x1120671 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01120674(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1120674); } r_01120674(cpu, 0x1120674); }
pub fn f_0112087a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x112087a); } r_01120674(cpu, 0x112087a); }
fn r_01120674(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1120674 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11206c4; continue; }
  pc = 0x1120686; continue;
 }
 0x1120686 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11206bb; continue; }
  pc = 0x11206aa; continue;
 }
 0x11206aa => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11206bb; continue;
 }
 0x11206bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11206c4; continue;
 }
 0x11206c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x11206e6; continue; }
  pc = 0x11206db; continue;
 }
 0x11206db => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x11206e6; continue;
 }
 0x11206e6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11206fd; continue;
 }
 0x11206ed => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11206fd; continue;
 }
 0x11206fd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11206ed; continue; }
  pc = 0x1120700; continue;
 }
 0x1120700 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11206e6; continue; }
  pc = 0x1120709; continue;
 }
 0x1120709 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120720; continue;
 }
 0x1120710 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120720; continue;
 }
 0x1120720 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120710; continue; }
  pc = 0x1120723; continue;
 }
 0x1120723 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120791; continue; }
  pc = 0x1120753; continue;
 }
 0x1120753 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120788; continue; }
  pc = 0x1120777; continue;
 }
 0x1120777 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120788; continue;
 }
 0x1120788 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120791; continue;
 }
 0x1120791 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x11207af; continue;
 }
 0x11207af => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11207c6; continue;
 }
 0x11207b6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11207c6; continue;
 }
 0x11207c6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11207b6; continue; }
  pc = 0x11207c9; continue;
 }
 0x11207c9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11207af; continue; }
  pc = 0x11207d2; continue;
 }
 0x11207d2 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x11207e9; continue;
 }
 0x11207d9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11207e9; continue;
 }
 0x11207e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11207d9; continue; }
  pc = 0x11207ec; continue;
 }
 0x11207ec => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120841; continue; }
  pc = 0x1120803; continue;
 }
 0x1120803 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120838; continue; }
  pc = 0x1120827; continue;
 }
 0x1120827 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120838; continue;
 }
 0x1120838 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120841; continue;
 }
 0x1120841 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x112085e; continue;
 }
 0x112084e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112085e; continue;
 }
 0x112085e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112084e; continue; }
  pc = 0x1120861; continue;
 }
 0x1120861 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x112087a; continue;
 }
 0x112087a => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x112087f; continue;
 }
 0x112087f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120896; continue;
 }
 0x1120886 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120896; continue;
 }
 0x1120896 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120886; continue; }
  pc = 0x1120899; continue;
 }
 0x1120899 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112087f; continue; }
  pc = 0x11208a2; continue;
 }
 0x11208a2 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11208b9; continue;
 }
 0x11208a9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11208b9; continue;
 }
 0x11208b9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11208a9; continue; }
  pc = 0x11208bc; continue;
 }
 0x11208bc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112092a; continue; }
  pc = 0x11208ec; continue;
 }
 0x11208ec => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120921; continue; }
  pc = 0x1120910; continue;
 }
 0x1120910 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120921; continue;
 }
 0x1120921 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112092a; continue;
 }
 0x112092a => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1120948; continue;
 }
 0x1120948 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x112095f; continue;
 }
 0x112094f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112095f; continue;
 }
 0x112095f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112094f; continue; }
  pc = 0x1120962; continue;
 }
 0x1120962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120948; continue; }
  pc = 0x112096b; continue;
 }
 0x112096b => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x1120982; continue;
 }
 0x1120972 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120982; continue;
 }
 0x1120982 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120972; continue; }
  pc = 0x1120985; continue;
 }
 0x1120985 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11209da; continue; }
  pc = 0x112099c; continue;
 }
 0x112099c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11209d1; continue; }
  pc = 0x11209c0; continue;
 }
 0x11209c0 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11209d1; continue;
 }
 0x11209d1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11209da; continue;
 }
 0x11209da => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x11209f7; continue;
 }
 0x11209e7 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11209f7; continue;
 }
 0x11209f7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11209e7; continue; }
  pc = 0x11209fa; continue;
 }
 0x11209fa => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1120a18; continue;
 }
 0x1120a18 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120a2f; continue;
 }
 0x1120a1f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120a2f; continue;
 }
 0x1120a2f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120a1f; continue; }
  pc = 0x1120a32; continue;
 }
 0x1120a32 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120a18; continue; }
  pc = 0x1120a3b; continue;
 }
 0x1120a3b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120a90; continue; }
  pc = 0x1120a52; continue;
 }
 0x1120a52 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120a87; continue; }
  pc = 0x1120a76; continue;
 }
 0x1120a76 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120a87; continue;
 }
 0x1120a87 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120a90; continue;
 }
 0x1120a90 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1120a9b; continue;
 }
 0x1120a9b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120ab2; continue;
 }
 0x1120aa2 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120ab2; continue;
 }
 0x1120ab2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120aa2; continue; }
  pc = 0x1120ab5; continue;
 }
 0x1120ab5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120a9b; continue; }
  pc = 0x1120abe; continue;
 }
 0x1120abe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120ad5; continue;
 }
 0x1120ac5 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120ad5; continue;
 }
 0x1120ad5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120ac5; continue; }
  pc = 0x1120ad8; continue;
 }
 0x1120ad8 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120b46; continue; }
  pc = 0x1120b08; continue;
 }
 0x1120b08 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120b3d; continue; }
  pc = 0x1120b2c; continue;
 }
 0x1120b2c => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120b3d; continue;
 }
 0x1120b3d => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120b46; continue;
 }
 0x1120b46 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1120b64; continue;
 }
 0x1120b64 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120b7b; continue;
 }
 0x1120b6b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120b7b; continue;
 }
 0x1120b7b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120b6b; continue; }
  pc = 0x1120b7e; continue;
 }
 0x1120b7e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120b64; continue; }
  pc = 0x1120b87; continue;
 }
 0x1120b87 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1120b9e; continue;
 }
 0x1120b8e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120b9e; continue;
 }
 0x1120b9e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120b8e; continue; }
  pc = 0x1120ba1; continue;
 }
 0x1120ba1 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120bf6; continue; }
  pc = 0x1120bb8; continue;
 }
 0x1120bb8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120bed; continue; }
  pc = 0x1120bdc; continue;
 }
 0x1120bdc => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120bed; continue;
 }
 0x1120bed => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120bf6; continue;
 }
 0x1120bf6 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x1120c13; continue;
 }
 0x1120c03 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120c13; continue;
 }
 0x1120c13 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120c03; continue; }
  pc = 0x1120c16; continue;
 }
 0x1120c16 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1120d32; continue; }
  pc = 0x1120c3f; continue;
 }
 0x1120c3f => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x1120c44; continue;
 }
 0x1120c44 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120c5b; continue;
 }
 0x1120c4b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120c5b; continue;
 }
 0x1120c5b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120c4b; continue; }
  pc = 0x1120c5e; continue;
 }
 0x1120c5e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120c44; continue; }
  pc = 0x1120c67; continue;
 }
 0x1120c67 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120c7e; continue;
 }
 0x1120c6e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120c7e; continue;
 }
 0x1120c7e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120c6e; continue; }
  pc = 0x1120c81; continue;
 }
 0x1120c81 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120cef; continue; }
  pc = 0x1120cb1; continue;
 }
 0x1120cb1 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120ce6; continue; }
  pc = 0x1120cd5; continue;
 }
 0x1120cd5 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120ce6; continue;
 }
 0x1120ce6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120cef; continue;
 }
 0x1120cef => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x1120d0d; continue;
 }
 0x1120d0d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120d24; continue;
 }
 0x1120d14 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120d24; continue;
 }
 0x1120d24 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120d14; continue; }
  pc = 0x1120d27; continue;
 }
 0x1120d27 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120d0d; continue; }
  pc = 0x1120d30; continue;
 }
 0x1120d30 => {
  pc = 0x1120d5a; continue;
 }
 0x1120d32 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x1120d37; continue;
 }
 0x1120d37 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1120d4e; continue;
 }
 0x1120d3e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1120d4e; continue;
 }
 0x1120d4e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120d3e; continue; }
  pc = 0x1120d51; continue;
 }
 0x1120d51 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120d37; continue; }
  pc = 0x1120d5a; continue;
 }
 0x1120d5a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01120d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1120d5c); }
 let mut pc: u32 = 0x1120d5c;
 loop { match pc {
 0x1120d5c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1120d7b; continue; }
  pc = 0x1120d78; continue;
 }
 0x1120d78 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x1120d7b; continue;
 }
 0x1120d7b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120de5; continue; }
  pc = 0x1120da7; continue;
 }
 0x1120da7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120ddc; continue; }
  pc = 0x1120dcb; continue;
 }
 0x1120dcb => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120ddc; continue;
 }
 0x1120ddc => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120de5; continue;
 }
 0x1120de5 => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1120e20; continue;
 }
 0x1120e20 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x1120e47; continue; }
  pc = 0x1120e28; continue;
 }
 0x1120e28 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1120e36; continue;
 }
 0x1120e2d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1120e36; continue;
 }
 0x1120e36 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e2d; continue; }
  pc = 0x1120e39; continue;
 }
 0x1120e39 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e20; continue; }
  pc = 0x1120e42; continue;
 }
 0x1120e42 => {
  pc = 0x112110c; continue;
 }
 0x1120e47 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1120f13; continue; }
  pc = 0x1120e58; continue;
 }
 0x1120e58 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1120e66; continue;
 }
 0x1120e5d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1120e66; continue;
 }
 0x1120e66 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e5d; continue; }
  pc = 0x1120e69; continue;
 }
 0x1120e69 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120ec0; continue; }
  pc = 0x1120e82; continue;
 }
 0x1120e82 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120eb7; continue; }
  pc = 0x1120ea6; continue;
 }
 0x1120ea6 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120eb7; continue;
 }
 0x1120eb7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120ec0; continue;
 }
 0x1120ec0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1120ee0; continue;
 }
 0x1120ed7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1120ee0; continue;
 }
 0x1120ee0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120ed7; continue; }
  pc = 0x1120ee3; continue;
 }
 0x1120ee3 => {
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
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e20; continue; }
  pc = 0x1120f0e; continue;
 }
 0x1120f0e => {
  pc = 0x112110c; continue;
 }
 0x1120f13 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x112106f; continue; }
  pc = 0x1120f21; continue;
 }
 0x1120f21 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1120feb; continue; }
  pc = 0x1120f2a; continue;
 }
 0x1120f2a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1120f38; continue;
 }
 0x1120f2f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1120f38; continue;
 }
 0x1120f38 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120f2f; continue; }
  pc = 0x1120f3b; continue;
 }
 0x1120f3b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1120fb2; continue; }
  pc = 0x1120f74; continue;
 }
 0x1120f74 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1120fa9; continue; }
  pc = 0x1120f98; continue;
 }
 0x1120f98 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1120fa9; continue;
 }
 0x1120fa9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1120fb2; continue;
 }
 0x1120fb2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e20; continue; }
  pc = 0x1120fe6; continue;
 }
 0x1120fe6 => {
  pc = 0x112110c; continue;
 }
 0x1120feb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1120ff9; continue;
 }
 0x1120ff0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1120ff9; continue;
 }
 0x1120ff9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120ff0; continue; }
  pc = 0x1120ffc; continue;
 }
 0x1120ffc => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121053; continue; }
  pc = 0x1121015; continue;
 }
 0x1121015 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112104a; continue; }
  pc = 0x1121039; continue;
 }
 0x1121039 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112104a; continue;
 }
 0x112104a => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121053; continue;
 }
 0x1121053 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e20; continue; }
  pc = 0x112106a; continue;
 }
 0x112106a => {
  pc = 0x112110c; continue;
 }
 0x112106f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x112107d; continue;
 }
 0x1121074 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x112107d; continue;
 }
 0x112107d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121074; continue; }
  pc = 0x1121080; continue;
 }
 0x1121080 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11210dd; continue; }
  pc = 0x112109f; continue;
 }
 0x112109f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11210d4; continue; }
  pc = 0x11210c3; continue;
 }
 0x11210c3 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11210d4; continue;
 }
 0x11210d4 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11210dd; continue;
 }
 0x11210dd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1120e20; continue; }
  pc = 0x112110c; continue;
 }
 0x112110c => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112110f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112110f); }
 let mut pc: u32 = 0x112110f;
 loop { match pc {
 0x112110f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x112112e; continue; }
  pc = 0x112112b; continue;
 }
 0x112112b => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x112112e; continue;
 }
 0x112112e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121198; continue; }
  pc = 0x112115a; continue;
 }
 0x112115a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112118f; continue; }
  pc = 0x112117e; continue;
 }
 0x112117e => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112118f; continue;
 }
 0x112118f => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121198; continue;
 }
 0x1121198 => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x11211d3; continue;
 }
 0x11211d3 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x11211f4; continue; }
  pc = 0x11211db; continue;
 }
 0x11211db => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11211e3; continue;
 }
 0x11211e0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11211e3; continue;
 }
 0x11211e3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211e0; continue; }
  pc = 0x11211e6; continue;
 }
 0x11211e6 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211d3; continue; }
  pc = 0x11211ef; continue;
 }
 0x11211ef => {
  pc = 0x112148b; continue;
 }
 0x11211f4 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11212b0; continue; }
  pc = 0x1121205; continue;
 }
 0x1121205 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x112120d; continue;
 }
 0x112120a => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x112120d; continue;
 }
 0x112120d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112120a; continue; }
  pc = 0x1121210; continue;
 }
 0x1121210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121267; continue; }
  pc = 0x1121229; continue;
 }
 0x1121229 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112125e; continue; }
  pc = 0x112124d; continue;
 }
 0x112124d => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112125e; continue;
 }
 0x112125e => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121267; continue;
 }
 0x1121267 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1121281; continue;
 }
 0x112127e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1121281; continue;
 }
 0x1121281 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112127e; continue; }
  pc = 0x1121284; continue;
 }
 0x1121284 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121295; continue;
 }
 0x1121292 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1121295; continue;
 }
 0x1121295 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121292; continue; }
  pc = 0x1121298; continue;
 }
 0x1121298 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211d3; continue; }
  pc = 0x11212ab; continue;
 }
 0x11212ab => {
  pc = 0x112148b; continue;
 }
 0x11212b0 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x11213f8; continue; }
  pc = 0x11212be; continue;
 }
 0x11212be => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x112137a; continue; }
  pc = 0x11212c7; continue;
 }
 0x11212c7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11212cf; continue;
 }
 0x11212cc => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11212cf; continue;
 }
 0x11212cf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11212cc; continue; }
  pc = 0x11212d2; continue;
 }
 0x11212d2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x11212eb; continue;
 }
 0x11212e8 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11212eb; continue;
 }
 0x11212eb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11212e8; continue; }
  pc = 0x11212ee; continue;
 }
 0x11212ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121345; continue; }
  pc = 0x1121307; continue;
 }
 0x1121307 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112133c; continue; }
  pc = 0x112132b; continue;
 }
 0x112132b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112133c; continue;
 }
 0x112133c => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121345; continue;
 }
 0x1121345 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x112135f; continue;
 }
 0x112135c => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x112135f; continue;
 }
 0x112135f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112135c; continue; }
  pc = 0x1121362; continue;
 }
 0x1121362 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211d3; continue; }
  pc = 0x1121375; continue;
 }
 0x1121375 => {
  pc = 0x112148b; continue;
 }
 0x112137a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121382; continue;
 }
 0x112137f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1121382; continue;
 }
 0x1121382 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112137f; continue; }
  pc = 0x1121385; continue;
 }
 0x1121385 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11213dc; continue; }
  pc = 0x112139e; continue;
 }
 0x112139e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11213d3; continue; }
  pc = 0x11213c2; continue;
 }
 0x11213c2 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11213d3; continue;
 }
 0x11213d3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11213dc; continue;
 }
 0x11213dc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211d3; continue; }
  pc = 0x11213f3; continue;
 }
 0x11213f3 => {
  pc = 0x112148b; continue;
 }
 0x11213f8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121400; continue;
 }
 0x11213fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1121400; continue;
 }
 0x1121400 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11213fd; continue; }
  pc = 0x1121403; continue;
 }
 0x1121403 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121460; continue; }
  pc = 0x1121422; continue;
 }
 0x1121422 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121457; continue; }
  pc = 0x1121446; continue;
 }
 0x1121446 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121457; continue;
 }
 0x1121457 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121460; continue;
 }
 0x1121460 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x112147b; continue;
 }
 0x1121478 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x112147b; continue;
 }
 0x112147b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121478; continue; }
  pc = 0x112147e; continue;
 }
 0x112147e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11211d3; continue; }
  pc = 0x112148b; continue;
 }
 0x112148b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112148e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112148e); }
 let mut pc: u32 = 0x112148e;
 loop { match pc {
 0x112148e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11214de; continue; }
  pc = 0x11214a0; continue;
 }
 0x11214a0 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11214d5; continue; }
  pc = 0x11214c4; continue;
 }
 0x11214c4 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11214d5; continue;
 }
 0x11214d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11214de; continue;
 }
 0x11214de => {
  { let v: u32 = cpu.edi; cpu.w32(0x1300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1121500; continue; }
  pc = 0x11214f5; continue;
 }
 0x11214f5 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x1121500; continue;
 }
 0x1121500 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121523; continue;
 }
 0x1121507 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121523; continue;
 }
 0x1121523 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121507; continue; }
  pc = 0x1121526; continue;
 }
 0x1121526 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121500; continue; }
  pc = 0x112152f; continue;
 }
 0x112152f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121546; continue;
 }
 0x1121536 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121546; continue;
 }
 0x1121546 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121536; continue; }
  pc = 0x1121549; continue;
 }
 0x1121549 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1121566; continue;
 }
 0x1121556 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121566; continue;
 }
 0x1121566 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121556; continue; }
  pc = 0x1121569; continue;
 }
 0x1121569 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11215be; continue; }
  pc = 0x1121580; continue;
 }
 0x1121580 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11215b5; continue; }
  pc = 0x11215a4; continue;
 }
 0x11215a4 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11215b5; continue;
 }
 0x11215b5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11215be; continue;
 }
 0x11215be => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x11215db; continue;
 }
 0x11215cb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11215db; continue;
 }
 0x11215db => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11215cb; continue; }
  pc = 0x11215de; continue;
 }
 0x11215de => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x11215e3; continue;
 }
 0x11215e3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121606; continue;
 }
 0x11215ea => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121606; continue;
 }
 0x1121606 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11215ea; continue; }
  pc = 0x1121609; continue;
 }
 0x1121609 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11215e3; continue; }
  pc = 0x1121612; continue;
 }
 0x1121612 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1121629; continue;
 }
 0x1121619 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121629; continue;
 }
 0x1121629 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121619; continue; }
  pc = 0x112162c; continue;
 }
 0x112162c => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121681; continue; }
  pc = 0x1121643; continue;
 }
 0x1121643 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121678; continue; }
  pc = 0x1121667; continue;
 }
 0x1121667 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121678; continue;
 }
 0x1121678 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121681; continue;
 }
 0x1121681 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x112169e; continue;
 }
 0x112168e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112169e; continue;
 }
 0x112169e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112168e; continue; }
  pc = 0x11216a1; continue;
 }
 0x11216a1 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11216be; continue;
 }
 0x11216ae => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11216be; continue;
 }
 0x11216be => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11216ae; continue; }
  pc = 0x11216c1; continue;
 }
 0x11216c1 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x11216c6; continue;
 }
 0x11216c6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11216e9; continue;
 }
 0x11216cd => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11216e9; continue;
 }
 0x11216e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11216cd; continue; }
  pc = 0x11216ec; continue;
 }
 0x11216ec => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11216c6; continue; }
  pc = 0x11216f5; continue;
 }
 0x11216f5 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x112170c; continue;
 }
 0x11216fc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112170c; continue;
 }
 0x112170c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11216fc; continue; }
  pc = 0x112170f; continue;
 }
 0x112170f => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x112172c; continue;
 }
 0x112171c => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112172c; continue;
 }
 0x112172c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112171c; continue; }
  pc = 0x112172f; continue;
 }
 0x112172f => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121784; continue; }
  pc = 0x1121746; continue;
 }
 0x1121746 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112177b; continue; }
  pc = 0x112176a; continue;
 }
 0x112176a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112177b; continue;
 }
 0x112177b => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121784; continue;
 }
 0x1121784 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x11217a1; continue;
 }
 0x1121791 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11217a1; continue;
 }
 0x11217a1 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121791; continue; }
  pc = 0x11217a4; continue;
 }
 0x11217a4 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x11217a9; continue;
 }
 0x11217a9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11217cc; continue;
 }
 0x11217b0 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11217cc; continue;
 }
 0x11217cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11217b0; continue; }
  pc = 0x11217cf; continue;
 }
 0x11217cf => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11217a9; continue; }
  pc = 0x11217d8; continue;
 }
 0x11217d8 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x11217ef; continue;
 }
 0x11217df => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11217ef; continue;
 }
 0x11217ef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11217df; continue; }
  pc = 0x11217f2; continue;
 }
 0x11217f2 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121847; continue; }
  pc = 0x1121809; continue;
 }
 0x1121809 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112183e; continue; }
  pc = 0x112182d; continue;
 }
 0x112182d => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112183e; continue;
 }
 0x112183e => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121847; continue;
 }
 0x1121847 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1121864; continue;
 }
 0x1121854 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121864; continue;
 }
 0x1121864 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121854; continue; }
  pc = 0x1121867; continue;
 }
 0x1121867 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121884; continue;
 }
 0x1121874 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121884; continue;
 }
 0x1121884 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121874; continue; }
  pc = 0x1121887; continue;
 }
 0x1121887 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x112188c; continue;
 }
 0x112188c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x11218af; continue;
 }
 0x1121893 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11218af; continue;
 }
 0x11218af => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121893; continue; }
  pc = 0x11218b2; continue;
 }
 0x11218b2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112188c; continue; }
  pc = 0x11218bb; continue;
 }
 0x11218bb => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121910; continue; }
  pc = 0x11218d2; continue;
 }
 0x11218d2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121907; continue; }
  pc = 0x11218f6; continue;
 }
 0x11218f6 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121907; continue;
 }
 0x1121907 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121910; continue;
 }
 0x1121910 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x112191b; continue;
 }
 0x112191b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x112193e; continue;
 }
 0x1121922 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x112193e; continue;
 }
 0x112193e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121922; continue; }
  pc = 0x1121941; continue;
 }
 0x1121941 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112191b; continue; }
  pc = 0x112194a; continue;
 }
 0x112194a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121961; continue;
 }
 0x1121951 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121961; continue;
 }
 0x1121961 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121951; continue; }
  pc = 0x1121964; continue;
 }
 0x1121964 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x1121981; continue;
 }
 0x1121971 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121981; continue;
 }
 0x1121981 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121971; continue; }
  pc = 0x1121984; continue;
 }
 0x1121984 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11219d9; continue; }
  pc = 0x112199b; continue;
 }
 0x112199b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11219d0; continue; }
  pc = 0x11219bf; continue;
 }
 0x11219bf => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11219d0; continue;
 }
 0x11219d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11219d9; continue;
 }
 0x11219d9 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x11219f6; continue;
 }
 0x11219e6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x11219f6; continue;
 }
 0x11219f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11219e6; continue; }
  pc = 0x11219f9; continue;
 }
 0x11219f9 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x11219fe; continue;
 }
 0x11219fe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121a21; continue;
 }
 0x1121a05 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121a21; continue;
 }
 0x1121a21 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121a05; continue; }
  pc = 0x1121a24; continue;
 }
 0x1121a24 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11219fe; continue; }
  pc = 0x1121a2d; continue;
 }
 0x1121a2d => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1121a44; continue;
 }
 0x1121a34 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121a44; continue;
 }
 0x1121a44 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121a34; continue; }
  pc = 0x1121a47; continue;
 }
 0x1121a47 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121a9c; continue; }
  pc = 0x1121a5e; continue;
 }
 0x1121a5e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121a93; continue; }
  pc = 0x1121a82; continue;
 }
 0x1121a82 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121a93; continue;
 }
 0x1121a93 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121a9c; continue;
 }
 0x1121a9c => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x1121ab9; continue;
 }
 0x1121aa9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121ab9; continue;
 }
 0x1121ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121aa9; continue; }
  pc = 0x1121abc; continue;
 }
 0x1121abc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121ad9; continue;
 }
 0x1121ac9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121ad9; continue;
 }
 0x1121ad9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121ac9; continue; }
  pc = 0x1121adc; continue;
 }
 0x1121adc => {
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1121c05; continue; }
  pc = 0x1121aec; continue;
 }
 0x1121aec => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x1121af1; continue;
 }
 0x1121af1 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121b14; continue;
 }
 0x1121af8 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121b14; continue;
 }
 0x1121b14 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121af8; continue; }
  pc = 0x1121b17; continue;
 }
 0x1121b17 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121af1; continue; }
  pc = 0x1121b20; continue;
 }
 0x1121b20 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121b37; continue;
 }
 0x1121b27 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121b37; continue;
 }
 0x1121b37 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121b27; continue; }
  pc = 0x1121b3a; continue;
 }
 0x1121b3a => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x1121b57; continue;
 }
 0x1121b47 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121b57; continue;
 }
 0x1121b57 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121b47; continue; }
  pc = 0x1121b5a; continue;
 }
 0x1121b5a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121baf; continue; }
  pc = 0x1121b71; continue;
 }
 0x1121b71 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121ba6; continue; }
  pc = 0x1121b95; continue;
 }
 0x1121b95 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121ba6; continue;
 }
 0x1121ba6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121baf; continue;
 }
 0x1121baf => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x1121bcc; continue;
 }
 0x1121bbc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121bcc; continue;
 }
 0x1121bcc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121bbc; continue; }
  pc = 0x1121bcf; continue;
 }
 0x1121bcf => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x1121bd4; continue;
 }
 0x1121bd4 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121bf7; continue;
 }
 0x1121bdb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121bf7; continue;
 }
 0x1121bf7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121bdb; continue; }
  pc = 0x1121bfa; continue;
 }
 0x1121bfa => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121bd4; continue; }
  pc = 0x1121c03; continue;
 }
 0x1121c03 => {
  pc = 0x1121c39; continue;
 }
 0x1121c05 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x1121c0a; continue;
 }
 0x1121c0a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x1121c2d; continue;
 }
 0x1121c11 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1121c2d; continue;
 }
 0x1121c2d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121c11; continue; }
  pc = 0x1121c30; continue;
 }
 0x1121c30 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121c0a; continue; }
  pc = 0x1121c39; continue;
 }
 0x1121c39 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01121c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1121c3b); }
 let mut pc: u32 = 0x1121c3b;
 loop { match pc {
 0x1121c3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1121c5a; continue; }
  pc = 0x1121c57; continue;
 }
 0x1121c57 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x1121c5a; continue;
 }
 0x1121c5a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121cc4; continue; }
  pc = 0x1121c86; continue;
 }
 0x1121c86 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121cbb; continue; }
  pc = 0x1121caa; continue;
 }
 0x1121caa => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121cbb; continue;
 }
 0x1121cbb => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121cc4; continue;
 }
 0x1121cc4 => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1121cff; continue;
 }
 0x1121cff => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x1121d2c; continue; }
  pc = 0x1121d07; continue;
 }
 0x1121d07 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121d1b; continue;
 }
 0x1121d0c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1121d1b; continue;
 }
 0x1121d1b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121d0c; continue; }
  pc = 0x1121d1e; continue;
 }
 0x1121d1e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121cff; continue; }
  pc = 0x1121d27; continue;
 }
 0x1121d27 => {
  pc = 0x1121fff; continue;
 }
 0x1121d2c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1121dfa; continue; }
  pc = 0x1121d3d; continue;
 }
 0x1121d3d => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1121d4b; continue;
 }
 0x1121d42 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121d4b; continue;
 }
 0x1121d4b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121d42; continue; }
  pc = 0x1121d4e; continue;
 }
 0x1121d4e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121da5; continue; }
  pc = 0x1121d67; continue;
 }
 0x1121d67 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121d9c; continue; }
  pc = 0x1121d8b; continue;
 }
 0x1121d8b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121d9c; continue;
 }
 0x1121d9c => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121da5; continue;
 }
 0x1121da5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1121dc5; continue;
 }
 0x1121dbc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121dc5; continue;
 }
 0x1121dc5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121dbc; continue; }
  pc = 0x1121dc8; continue;
 }
 0x1121dc8 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121ddf; continue;
 }
 0x1121dd6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121ddf; continue;
 }
 0x1121ddf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121dd6; continue; }
  pc = 0x1121de2; continue;
 }
 0x1121de2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121cff; continue; }
  pc = 0x1121df5; continue;
 }
 0x1121df5 => {
  pc = 0x1121fff; continue;
 }
 0x1121dfa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x1121f60; continue; }
  pc = 0x1121e08; continue;
 }
 0x1121e08 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1121ed6; continue; }
  pc = 0x1121e11; continue;
 }
 0x1121e11 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121e1f; continue;
 }
 0x1121e16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121e1f; continue;
 }
 0x1121e1f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121e16; continue; }
  pc = 0x1121e22; continue;
 }
 0x1121e22 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1121e41; continue;
 }
 0x1121e38 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121e41; continue;
 }
 0x1121e41 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121e38; continue; }
  pc = 0x1121e44; continue;
 }
 0x1121e44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121e9b; continue; }
  pc = 0x1121e5d; continue;
 }
 0x1121e5d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121e92; continue; }
  pc = 0x1121e81; continue;
 }
 0x1121e81 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121e92; continue;
 }
 0x1121e92 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121e9b; continue;
 }
 0x1121e9b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1121ebb; continue;
 }
 0x1121eb2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121ebb; continue;
 }
 0x1121ebb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121eb2; continue; }
  pc = 0x1121ebe; continue;
 }
 0x1121ebe => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121cff; continue; }
  pc = 0x1121ed1; continue;
 }
 0x1121ed1 => {
  pc = 0x1121fff; continue;
 }
 0x1121ed6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121eea; continue;
 }
 0x1121edb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x1121eea; continue;
 }
 0x1121eea => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121edb; continue; }
  pc = 0x1121eed; continue;
 }
 0x1121eed => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121f44; continue; }
  pc = 0x1121f06; continue;
 }
 0x1121f06 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121f3b; continue; }
  pc = 0x1121f2a; continue;
 }
 0x1121f2a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121f3b; continue;
 }
 0x1121f3b => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121f44; continue;
 }
 0x1121f44 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121cff; continue; }
  pc = 0x1121f5b; continue;
 }
 0x1121f5b => {
  pc = 0x1121fff; continue;
 }
 0x1121f60 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121f6e; continue;
 }
 0x1121f65 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121f6e; continue;
 }
 0x1121f6e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121f65; continue; }
  pc = 0x1121f71; continue;
 }
 0x1121f71 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1121fce; continue; }
  pc = 0x1121f90; continue;
 }
 0x1121f90 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1121fc5; continue; }
  pc = 0x1121fb4; continue;
 }
 0x1121fb4 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1121fc5; continue;
 }
 0x1121fc5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1121fce; continue;
 }
 0x1121fce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1121fef; continue;
 }
 0x1121fe6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x1121fef; continue;
 }
 0x1121fef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121fe6; continue; }
  pc = 0x1121ff2; continue;
 }
 0x1121ff2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1121cff; continue; }
  pc = 0x1121fff; continue;
 }
 0x1121fff => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01122002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1122002); }
 let mut pc: u32 = 0x1122002;
 loop { match pc {
 0x1122002 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1122021; continue; }
  pc = 0x112201e; continue;
 }
 0x112201e => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x1122021; continue;
 }
 0x1122021 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112208b; continue; }
  pc = 0x112204d; continue;
 }
 0x112204d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1122082; continue; }
  pc = 0x1122071; continue;
 }
 0x1122071 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1122082; continue;
 }
 0x1122082 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112208b; continue;
 }
 0x112208b => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x11220c6; continue;
 }
 0x11220c6 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x11220e7; continue; }
  pc = 0x11220ce; continue;
 }
 0x11220ce => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11220d6; continue;
 }
 0x11220d3 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11220d6; continue;
 }
 0x11220d6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220d3; continue; }
  pc = 0x11220d9; continue;
 }
 0x11220d9 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220c6; continue; }
  pc = 0x11220e2; continue;
 }
 0x11220e2 => {
  pc = 0x112237e; continue;
 }
 0x11220e7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11221a3; continue; }
  pc = 0x11220f8; continue;
 }
 0x11220f8 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1122100; continue;
 }
 0x11220fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1122100; continue;
 }
 0x1122100 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220fd; continue; }
  pc = 0x1122103; continue;
 }
 0x1122103 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112215a; continue; }
  pc = 0x112211c; continue;
 }
 0x112211c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1122151; continue; }
  pc = 0x1122140; continue;
 }
 0x1122140 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1122151; continue;
 }
 0x1122151 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112215a; continue;
 }
 0x112215a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1122174; continue;
 }
 0x1122171 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1122174; continue;
 }
 0x1122174 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122171; continue; }
  pc = 0x1122177; continue;
 }
 0x1122177 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1122188; continue;
 }
 0x1122185 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1122188; continue;
 }
 0x1122188 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122185; continue; }
  pc = 0x112218b; continue;
 }
 0x112218b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220c6; continue; }
  pc = 0x112219e; continue;
 }
 0x112219e => {
  pc = 0x112237e; continue;
 }
 0x11221a3 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x11222eb; continue; }
  pc = 0x11221b1; continue;
 }
 0x11221b1 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x112226d; continue; }
  pc = 0x11221ba; continue;
 }
 0x11221ba => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11221c2; continue;
 }
 0x11221bf => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11221c2; continue;
 }
 0x11221c2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11221bf; continue; }
  pc = 0x11221c5; continue;
 }
 0x11221c5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x11221de; continue;
 }
 0x11221db => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11221de; continue;
 }
 0x11221de => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11221db; continue; }
  pc = 0x11221e1; continue;
 }
 0x11221e1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1122238; continue; }
  pc = 0x11221fa; continue;
 }
 0x11221fa => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112222f; continue; }
  pc = 0x112221e; continue;
 }
 0x112221e => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112222f; continue;
 }
 0x112222f => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1122238; continue;
 }
 0x1122238 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1122252; continue;
 }
 0x112224f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1122252; continue;
 }
 0x1122252 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112224f; continue; }
  pc = 0x1122255; continue;
 }
 0x1122255 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220c6; continue; }
  pc = 0x1122268; continue;
 }
 0x1122268 => {
  pc = 0x112237e; continue;
 }
 0x112226d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1122275; continue;
 }
 0x1122272 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1122275; continue;
 }
 0x1122275 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122272; continue; }
  pc = 0x1122278; continue;
 }
 0x1122278 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11222cf; continue; }
  pc = 0x1122291; continue;
 }
 0x1122291 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11222c6; continue; }
  pc = 0x11222b5; continue;
 }
 0x11222b5 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11222c6; continue;
 }
 0x11222c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11222cf; continue;
 }
 0x11222cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220c6; continue; }
  pc = 0x11222e6; continue;
 }
 0x11222e6 => {
  pc = 0x112237e; continue;
 }
 0x11222eb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11222f3; continue;
 }
 0x11222f0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x11222f3; continue;
 }
 0x11222f3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11222f0; continue; }
  pc = 0x11222f6; continue;
 }
 0x11222f6 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1122353; continue; }
  pc = 0x1122315; continue;
 }
 0x1122315 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112234a; continue; }
  pc = 0x1122339; continue;
 }
 0x1122339 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112234a; continue;
 }
 0x112234a => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1122353; continue;
 }
 0x1122353 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x112236e; continue;
 }
 0x112236b => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x112236e; continue;
 }
 0x112236e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112236b; continue; }
  pc = 0x1122371; continue;
 }
 0x1122371 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11220c6; continue; }
  pc = 0x112237e; continue;
 }
 0x112237e => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01122381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1122381); }
 let mut pc: u32 = 0x1122381;
 loop { match pc {
 0x1122381 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11223d1; continue; }
  pc = 0x1122393; continue;
 }
 0x1122393 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11223c8; continue; }
  pc = 0x11223b7; continue;
 }
 0x11223b7 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11223c8; continue;
 }
 0x11223c8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11223d1; continue;
 }
 0x11223d1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x1300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x11223f3; continue; }
  pc = 0x11223e8; continue;
 }
 0x11223e8 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x11223f3; continue;
 }
 0x11223f3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11223fb, "Movq");
  cpu.unimplemented(0x1122402, "Movq");
  pc = 0x1122405; continue;
 }
 0x1122405 => {
  cpu.unimplemented(0x1122405, "Movq");
  cpu.unimplemented(0x1122409, "Movq");
  cpu.unimplemented(0x112240c, "Movq");
  cpu.unimplemented(0x112240f, "Movq");
  cpu.unimplemented(0x1122412, "Psrlq");
  cpu.unimplemented(0x1122416, "Psllq");
  cpu.unimplemented(0x112241a, "Pand");
  cpu.unimplemented(0x1122421, "Por");
  cpu.unimplemented(0x1122424, "Psrlq");
  cpu.unimplemented(0x1122428, "Pand");
  cpu.unimplemented(0x112242f, "Psrlq");
  cpu.unimplemented(0x1122433, "Paddw");
  cpu.unimplemented(0x1122436, "Movq");
  cpu.unimplemented(0x1122439, "Punpcklwd");
  cpu.unimplemented(0x112243c, "Punpckhwd");
  cpu.unimplemented(0x112243f, "Movq");
  cpu.unimplemented(0x1122442, "Pand");
  cpu.unimplemented(0x1122445, "Movq");
  cpu.unimplemented(0x112244c, "Movq");
  cpu.unimplemented(0x1122450, "Pand");
  cpu.unimplemented(0x1122453, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122460, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122405; continue; }
  pc = 0x1122466; continue;
 }
 0x1122466 => {
  cpu.unimplemented(0x1122466, "Punpcklwd");
  cpu.unimplemented(0x1122469, "Punpckhwd");
  cpu.unimplemented(0x112246c, "Movq");
  cpu.unimplemented(0x112246f, "Pand");
  cpu.unimplemented(0x1122472, "Movq");
  cpu.unimplemented(0x1122479, "Movq");
  cpu.unimplemented(0x112247d, "Pand");
  cpu.unimplemented(0x1122480, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11223f3; continue; }
  pc = 0x112249a; continue;
 }
 0x112249a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11224a2, "Movq");
  cpu.unimplemented(0x11224a9, "Movq");
  pc = 0x11224ac; continue;
 }
 0x11224ac => {
  cpu.unimplemented(0x11224ac, "Movq");
  cpu.unimplemented(0x11224b0, "Movq");
  cpu.unimplemented(0x11224b3, "Movq");
  cpu.unimplemented(0x11224b6, "Movq");
  cpu.unimplemented(0x11224b9, "Psrlq");
  cpu.unimplemented(0x11224bd, "Psllq");
  cpu.unimplemented(0x11224c1, "Pand");
  cpu.unimplemented(0x11224c8, "Por");
  cpu.unimplemented(0x11224cb, "Psrlq");
  cpu.unimplemented(0x11224cf, "Pand");
  cpu.unimplemented(0x11224d6, "Psrlq");
  cpu.unimplemented(0x11224da, "Paddw");
  cpu.unimplemented(0x11224dd, "Movq");
  cpu.unimplemented(0x11224e0, "Punpcklwd");
  cpu.unimplemented(0x11224e3, "Punpckhwd");
  cpu.unimplemented(0x11224e6, "Movq");
  cpu.unimplemented(0x11224e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11224f3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11224ac; continue; }
  pc = 0x11224f9; continue;
 }
 0x11224f9 => {
  cpu.unimplemented(0x11224f9, "Punpcklwd");
  cpu.unimplemented(0x11224fc, "Punpckhwd");
  cpu.unimplemented(0x11224ff, "Movq");
  cpu.unimplemented(0x1122502, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x112251a, "Movq");
  cpu.unimplemented(0x1122521, "Movq");
  pc = 0x1122524; continue;
 }
 0x1122524 => {
  cpu.unimplemented(0x1122524, "Movq");
  cpu.unimplemented(0x1122528, "Movq");
  cpu.unimplemented(0x112252b, "Movq");
  cpu.unimplemented(0x112252e, "Movq");
  cpu.unimplemented(0x1122531, "Psrlq");
  cpu.unimplemented(0x1122535, "Psllq");
  cpu.unimplemented(0x1122539, "Pand");
  cpu.unimplemented(0x1122540, "Por");
  cpu.unimplemented(0x1122543, "Psrlq");
  cpu.unimplemented(0x1122547, "Pand");
  cpu.unimplemented(0x112254e, "Psrlq");
  cpu.unimplemented(0x1122552, "Paddw");
  cpu.unimplemented(0x1122555, "Movq");
  cpu.unimplemented(0x1122558, "Punpcklwd");
  cpu.unimplemented(0x112255b, "Punpckhwd");
  cpu.unimplemented(0x112255e, "Movq");
  cpu.unimplemented(0x1122561, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x112256b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122524; continue; }
  pc = 0x1122571; continue;
 }
 0x1122571 => {
  cpu.unimplemented(0x1122571, "Punpcklwd");
  cpu.unimplemented(0x1122574, "Punpckhwd");
  cpu.unimplemented(0x1122577, "Movq");
  cpu.unimplemented(0x112257a, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11225d9; continue; }
  pc = 0x112259b; continue;
 }
 0x112259b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11225d0; continue; }
  pc = 0x11225bf; continue;
 }
 0x11225bf => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11225d0; continue;
 }
 0x11225d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11225d9; continue;
 }
 0x11225d9 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11225e7, "Movq");
  cpu.unimplemented(0x11225ee, "Movq");
  pc = 0x11225f1; continue;
 }
 0x11225f1 => {
  cpu.unimplemented(0x11225f1, "Movq");
  cpu.unimplemented(0x11225f5, "Movq");
  cpu.unimplemented(0x11225f8, "Movq");
  cpu.unimplemented(0x11225fb, "Movq");
  cpu.unimplemented(0x11225fe, "Psrlq");
  cpu.unimplemented(0x1122602, "Psllq");
  cpu.unimplemented(0x1122606, "Pand");
  cpu.unimplemented(0x112260d, "Por");
  cpu.unimplemented(0x1122610, "Psrlq");
  cpu.unimplemented(0x1122614, "Pand");
  cpu.unimplemented(0x112261b, "Psrlq");
  cpu.unimplemented(0x112261f, "Paddw");
  cpu.unimplemented(0x1122622, "Movq");
  cpu.unimplemented(0x1122625, "Punpcklwd");
  cpu.unimplemented(0x1122628, "Punpckhwd");
  cpu.unimplemented(0x112262b, "Movq");
  cpu.unimplemented(0x112262e, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122638, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11225f1; continue; }
  pc = 0x112263e; continue;
 }
 0x112263e => {
  cpu.unimplemented(0x112263e, "Punpcklwd");
  cpu.unimplemented(0x1122641, "Punpckhwd");
  cpu.unimplemented(0x1122644, "Movq");
  cpu.unimplemented(0x1122647, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1122656; continue;
 }
 0x1122656 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x112265e, "Movq");
  cpu.unimplemented(0x1122665, "Movq");
  pc = 0x1122668; continue;
 }
 0x1122668 => {
  cpu.unimplemented(0x1122668, "Movq");
  cpu.unimplemented(0x112266c, "Movq");
  cpu.unimplemented(0x112266f, "Movq");
  cpu.unimplemented(0x1122672, "Movq");
  cpu.unimplemented(0x1122675, "Psrlq");
  cpu.unimplemented(0x1122679, "Psllq");
  cpu.unimplemented(0x112267d, "Pand");
  cpu.unimplemented(0x1122684, "Por");
  cpu.unimplemented(0x1122687, "Psrlq");
  cpu.unimplemented(0x112268b, "Pand");
  cpu.unimplemented(0x1122692, "Psrlq");
  cpu.unimplemented(0x1122696, "Paddw");
  cpu.unimplemented(0x1122699, "Movq");
  cpu.unimplemented(0x112269c, "Punpcklwd");
  cpu.unimplemented(0x112269f, "Punpckhwd");
  cpu.unimplemented(0x11226a2, "Movq");
  cpu.unimplemented(0x11226a5, "Pand");
  cpu.unimplemented(0x11226a8, "Movq");
  cpu.unimplemented(0x11226af, "Movq");
  cpu.unimplemented(0x11226b3, "Pand");
  cpu.unimplemented(0x11226b6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11226c3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122668; continue; }
  pc = 0x11226c9; continue;
 }
 0x11226c9 => {
  cpu.unimplemented(0x11226c9, "Punpcklwd");
  cpu.unimplemented(0x11226cc, "Punpckhwd");
  cpu.unimplemented(0x11226cf, "Movq");
  cpu.unimplemented(0x11226d2, "Pand");
  cpu.unimplemented(0x11226d5, "Movq");
  cpu.unimplemented(0x11226dc, "Movq");
  cpu.unimplemented(0x11226e0, "Pand");
  cpu.unimplemented(0x11226e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122656; continue; }
  pc = 0x11226fd; continue;
 }
 0x11226fd => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122705, "Movq");
  cpu.unimplemented(0x112270c, "Movq");
  pc = 0x112270f; continue;
 }
 0x112270f => {
  cpu.unimplemented(0x112270f, "Movq");
  cpu.unimplemented(0x1122713, "Movq");
  cpu.unimplemented(0x1122716, "Movq");
  cpu.unimplemented(0x1122719, "Movq");
  cpu.unimplemented(0x112271c, "Psrlq");
  cpu.unimplemented(0x1122720, "Psllq");
  cpu.unimplemented(0x1122724, "Pand");
  cpu.unimplemented(0x112272b, "Por");
  cpu.unimplemented(0x112272e, "Psrlq");
  cpu.unimplemented(0x1122732, "Pand");
  cpu.unimplemented(0x1122739, "Psrlq");
  cpu.unimplemented(0x112273d, "Paddw");
  cpu.unimplemented(0x1122740, "Movq");
  cpu.unimplemented(0x1122743, "Punpcklwd");
  cpu.unimplemented(0x1122746, "Punpckhwd");
  cpu.unimplemented(0x1122749, "Movq");
  cpu.unimplemented(0x112274c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122756, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x112270f; continue; }
  pc = 0x112275c; continue;
 }
 0x112275c => {
  cpu.unimplemented(0x112275c, "Punpcklwd");
  cpu.unimplemented(0x112275f, "Punpckhwd");
  cpu.unimplemented(0x1122762, "Movq");
  cpu.unimplemented(0x1122765, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11227c4; continue; }
  pc = 0x1122786; continue;
 }
 0x1122786 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11227bb; continue; }
  pc = 0x11227aa; continue;
 }
 0x11227aa => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11227bb; continue;
 }
 0x11227bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11227c4; continue;
 }
 0x11227c4 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11227d2, "Movq");
  cpu.unimplemented(0x11227d9, "Movq");
  pc = 0x11227dc; continue;
 }
 0x11227dc => {
  cpu.unimplemented(0x11227dc, "Movq");
  cpu.unimplemented(0x11227e0, "Movq");
  cpu.unimplemented(0x11227e3, "Movq");
  cpu.unimplemented(0x11227e6, "Movq");
  cpu.unimplemented(0x11227e9, "Psrlq");
  cpu.unimplemented(0x11227ed, "Psllq");
  cpu.unimplemented(0x11227f1, "Pand");
  cpu.unimplemented(0x11227f8, "Por");
  cpu.unimplemented(0x11227fb, "Psrlq");
  cpu.unimplemented(0x11227ff, "Pand");
  cpu.unimplemented(0x1122806, "Psrlq");
  cpu.unimplemented(0x112280a, "Paddw");
  cpu.unimplemented(0x112280d, "Movq");
  cpu.unimplemented(0x1122810, "Punpcklwd");
  cpu.unimplemented(0x1122813, "Punpckhwd");
  cpu.unimplemented(0x1122816, "Movq");
  cpu.unimplemented(0x1122819, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122823, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11227dc; continue; }
  pc = 0x1122829; continue;
 }
 0x1122829 => {
  cpu.unimplemented(0x1122829, "Punpcklwd");
  cpu.unimplemented(0x112282c, "Punpckhwd");
  cpu.unimplemented(0x112282f, "Movq");
  cpu.unimplemented(0x1122832, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x112284a, "Movq");
  cpu.unimplemented(0x1122851, "Movq");
  pc = 0x1122854; continue;
 }
 0x1122854 => {
  cpu.unimplemented(0x1122854, "Movq");
  cpu.unimplemented(0x1122858, "Movq");
  cpu.unimplemented(0x112285b, "Movq");
  cpu.unimplemented(0x112285e, "Movq");
  cpu.unimplemented(0x1122861, "Psrlq");
  cpu.unimplemented(0x1122865, "Psllq");
  cpu.unimplemented(0x1122869, "Pand");
  cpu.unimplemented(0x1122870, "Por");
  cpu.unimplemented(0x1122873, "Psrlq");
  cpu.unimplemented(0x1122877, "Pand");
  cpu.unimplemented(0x112287e, "Psrlq");
  cpu.unimplemented(0x1122882, "Paddw");
  cpu.unimplemented(0x1122885, "Movq");
  cpu.unimplemented(0x1122888, "Punpcklwd");
  cpu.unimplemented(0x112288b, "Punpckhwd");
  cpu.unimplemented(0x112288e, "Movq");
  cpu.unimplemented(0x1122891, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x112289b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122854; continue; }
  pc = 0x11228a1; continue;
 }
 0x11228a1 => {
  cpu.unimplemented(0x11228a1, "Punpcklwd");
  cpu.unimplemented(0x11228a4, "Punpckhwd");
  cpu.unimplemented(0x11228a7, "Movq");
  cpu.unimplemented(0x11228aa, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x11228b9; continue;
 }
 0x11228b9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11228c1, "Movq");
  cpu.unimplemented(0x11228c8, "Movq");
  pc = 0x11228cb; continue;
 }
 0x11228cb => {
  cpu.unimplemented(0x11228cb, "Movq");
  cpu.unimplemented(0x11228cf, "Movq");
  cpu.unimplemented(0x11228d2, "Movq");
  cpu.unimplemented(0x11228d5, "Movq");
  cpu.unimplemented(0x11228d8, "Psrlq");
  cpu.unimplemented(0x11228dc, "Psllq");
  cpu.unimplemented(0x11228e0, "Pand");
  cpu.unimplemented(0x11228e7, "Por");
  cpu.unimplemented(0x11228ea, "Psrlq");
  cpu.unimplemented(0x11228ee, "Pand");
  cpu.unimplemented(0x11228f5, "Psrlq");
  cpu.unimplemented(0x11228f9, "Paddw");
  cpu.unimplemented(0x11228fc, "Movq");
  cpu.unimplemented(0x11228ff, "Punpcklwd");
  cpu.unimplemented(0x1122902, "Punpckhwd");
  cpu.unimplemented(0x1122905, "Movq");
  cpu.unimplemented(0x1122908, "Pand");
  cpu.unimplemented(0x112290b, "Movq");
  cpu.unimplemented(0x1122912, "Movq");
  cpu.unimplemented(0x1122916, "Pand");
  cpu.unimplemented(0x1122919, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122926, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11228cb; continue; }
  pc = 0x112292c; continue;
 }
 0x112292c => {
  cpu.unimplemented(0x112292c, "Punpcklwd");
  cpu.unimplemented(0x112292f, "Punpckhwd");
  cpu.unimplemented(0x1122932, "Movq");
  cpu.unimplemented(0x1122935, "Pand");
  cpu.unimplemented(0x1122938, "Movq");
  cpu.unimplemented(0x112293f, "Movq");
  cpu.unimplemented(0x1122943, "Pand");
  cpu.unimplemented(0x1122946, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11228b9; continue; }
  pc = 0x1122960; continue;
 }
 0x1122960 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122968, "Movq");
  cpu.unimplemented(0x112296f, "Movq");
  pc = 0x1122972; continue;
 }
 0x1122972 => {
  cpu.unimplemented(0x1122972, "Movq");
  cpu.unimplemented(0x1122976, "Movq");
  cpu.unimplemented(0x1122979, "Movq");
  cpu.unimplemented(0x112297c, "Movq");
  cpu.unimplemented(0x112297f, "Psrlq");
  cpu.unimplemented(0x1122983, "Psllq");
  cpu.unimplemented(0x1122987, "Pand");
  cpu.unimplemented(0x112298e, "Por");
  cpu.unimplemented(0x1122991, "Psrlq");
  cpu.unimplemented(0x1122995, "Pand");
  cpu.unimplemented(0x112299c, "Psrlq");
  cpu.unimplemented(0x11229a0, "Paddw");
  cpu.unimplemented(0x11229a3, "Movq");
  cpu.unimplemented(0x11229a6, "Punpcklwd");
  cpu.unimplemented(0x11229a9, "Punpckhwd");
  cpu.unimplemented(0x11229ac, "Movq");
  cpu.unimplemented(0x11229af, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11229b9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122972; continue; }
  pc = 0x11229bf; continue;
 }
 0x11229bf => {
  cpu.unimplemented(0x11229bf, "Punpcklwd");
  cpu.unimplemented(0x11229c2, "Punpckhwd");
  cpu.unimplemented(0x11229c5, "Movq");
  cpu.unimplemented(0x11229c8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11229e0, "Movq");
  cpu.unimplemented(0x11229e7, "Movq");
  pc = 0x11229ea; continue;
 }
 0x11229ea => {
  cpu.unimplemented(0x11229ea, "Movq");
  cpu.unimplemented(0x11229ee, "Movq");
  cpu.unimplemented(0x11229f1, "Movq");
  cpu.unimplemented(0x11229f4, "Movq");
  cpu.unimplemented(0x11229f7, "Psrlq");
  cpu.unimplemented(0x11229fb, "Psllq");
  cpu.unimplemented(0x11229ff, "Pand");
  cpu.unimplemented(0x1122a06, "Por");
  cpu.unimplemented(0x1122a09, "Psrlq");
  cpu.unimplemented(0x1122a0d, "Pand");
  cpu.unimplemented(0x1122a14, "Psrlq");
  cpu.unimplemented(0x1122a18, "Paddw");
  cpu.unimplemented(0x1122a1b, "Movq");
  cpu.unimplemented(0x1122a1e, "Punpcklwd");
  cpu.unimplemented(0x1122a21, "Punpckhwd");
  cpu.unimplemented(0x1122a24, "Movq");
  cpu.unimplemented(0x1122a27, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122a31, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11229ea; continue; }
  pc = 0x1122a37; continue;
 }
 0x1122a37 => {
  cpu.unimplemented(0x1122a37, "Punpcklwd");
  cpu.unimplemented(0x1122a3a, "Punpckhwd");
  cpu.unimplemented(0x1122a3d, "Movq");
  cpu.unimplemented(0x1122a40, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1122a9f; continue; }
  pc = 0x1122a61; continue;
 }
 0x1122a61 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1122a96; continue; }
  pc = 0x1122a85; continue;
 }
 0x1122a85 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1122a96; continue;
 }
 0x1122a96 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1122a9f; continue;
 }
 0x1122a9f => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122aad, "Movq");
  cpu.unimplemented(0x1122ab4, "Movq");
  pc = 0x1122ab7; continue;
 }
 0x1122ab7 => {
  cpu.unimplemented(0x1122ab7, "Movq");
  cpu.unimplemented(0x1122abb, "Movq");
  cpu.unimplemented(0x1122abe, "Movq");
  cpu.unimplemented(0x1122ac1, "Movq");
  cpu.unimplemented(0x1122ac4, "Psrlq");
  cpu.unimplemented(0x1122ac8, "Psllq");
  cpu.unimplemented(0x1122acc, "Pand");
  cpu.unimplemented(0x1122ad3, "Por");
  cpu.unimplemented(0x1122ad6, "Psrlq");
  cpu.unimplemented(0x1122ada, "Pand");
  cpu.unimplemented(0x1122ae1, "Psrlq");
  cpu.unimplemented(0x1122ae5, "Paddw");
  cpu.unimplemented(0x1122ae8, "Movq");
  cpu.unimplemented(0x1122aeb, "Punpcklwd");
  cpu.unimplemented(0x1122aee, "Punpckhwd");
  cpu.unimplemented(0x1122af1, "Movq");
  cpu.unimplemented(0x1122af4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122afe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122ab7; continue; }
  pc = 0x1122b04; continue;
 }
 0x1122b04 => {
  cpu.unimplemented(0x1122b04, "Punpcklwd");
  cpu.unimplemented(0x1122b07, "Punpckhwd");
  cpu.unimplemented(0x1122b0a, "Movq");
  cpu.unimplemented(0x1122b0d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1122b1c; continue;
 }
 0x1122b1c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122b24, "Movq");
  cpu.unimplemented(0x1122b2b, "Movq");
  pc = 0x1122b2e; continue;
 }
 0x1122b2e => {
  cpu.unimplemented(0x1122b2e, "Movq");
  cpu.unimplemented(0x1122b32, "Movq");
  cpu.unimplemented(0x1122b35, "Movq");
  cpu.unimplemented(0x1122b38, "Movq");
  cpu.unimplemented(0x1122b3b, "Psrlq");
  cpu.unimplemented(0x1122b3f, "Psllq");
  cpu.unimplemented(0x1122b43, "Pand");
  cpu.unimplemented(0x1122b4a, "Por");
  cpu.unimplemented(0x1122b4d, "Psrlq");
  cpu.unimplemented(0x1122b51, "Pand");
  cpu.unimplemented(0x1122b58, "Psrlq");
  cpu.unimplemented(0x1122b5c, "Paddw");
  cpu.unimplemented(0x1122b5f, "Movq");
  cpu.unimplemented(0x1122b62, "Punpcklwd");
  cpu.unimplemented(0x1122b65, "Punpckhwd");
  cpu.unimplemented(0x1122b68, "Movq");
  cpu.unimplemented(0x1122b6b, "Pand");
  cpu.unimplemented(0x1122b6e, "Movq");
  cpu.unimplemented(0x1122b75, "Movq");
  cpu.unimplemented(0x1122b79, "Pand");
  cpu.unimplemented(0x1122b7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122b89, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122b2e; continue; }
  pc = 0x1122b8f; continue;
 }
 0x1122b8f => {
  cpu.unimplemented(0x1122b8f, "Punpcklwd");
  cpu.unimplemented(0x1122b92, "Punpckhwd");
  cpu.unimplemented(0x1122b95, "Movq");
  cpu.unimplemented(0x1122b98, "Pand");
  cpu.unimplemented(0x1122b9b, "Movq");
  cpu.unimplemented(0x1122ba2, "Movq");
  cpu.unimplemented(0x1122ba6, "Pand");
  cpu.unimplemented(0x1122ba9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122b1c; continue; }
  pc = 0x1122bc3; continue;
 }
 0x1122bc3 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122bcb, "Movq");
  cpu.unimplemented(0x1122bd2, "Movq");
  pc = 0x1122bd5; continue;
 }
 0x1122bd5 => {
  cpu.unimplemented(0x1122bd5, "Movq");
  cpu.unimplemented(0x1122bd9, "Movq");
  cpu.unimplemented(0x1122bdc, "Movq");
  cpu.unimplemented(0x1122bdf, "Movq");
  cpu.unimplemented(0x1122be2, "Psrlq");
  cpu.unimplemented(0x1122be6, "Psllq");
  cpu.unimplemented(0x1122bea, "Pand");
  cpu.unimplemented(0x1122bf1, "Por");
  cpu.unimplemented(0x1122bf4, "Psrlq");
  cpu.unimplemented(0x1122bf8, "Pand");
  cpu.unimplemented(0x1122bff, "Psrlq");
  cpu.unimplemented(0x1122c03, "Paddw");
  cpu.unimplemented(0x1122c06, "Movq");
  cpu.unimplemented(0x1122c09, "Punpcklwd");
  cpu.unimplemented(0x1122c0c, "Punpckhwd");
  cpu.unimplemented(0x1122c0f, "Movq");
  cpu.unimplemented(0x1122c12, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122c1c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122bd5; continue; }
  pc = 0x1122c22; continue;
 }
 0x1122c22 => {
  cpu.unimplemented(0x1122c22, "Punpcklwd");
  cpu.unimplemented(0x1122c25, "Punpckhwd");
  cpu.unimplemented(0x1122c28, "Movq");
  cpu.unimplemented(0x1122c2b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1122c8a; continue; }
  pc = 0x1122c4c; continue;
 }
 0x1122c4c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1122c81; continue; }
  pc = 0x1122c70; continue;
 }
 0x1122c70 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1122c81; continue;
 }
 0x1122c81 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1122c8a; continue;
 }
 0x1122c8a => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122c98, "Movq");
  cpu.unimplemented(0x1122c9f, "Movq");
  pc = 0x1122ca2; continue;
 }
 0x1122ca2 => {
  cpu.unimplemented(0x1122ca2, "Movq");
  cpu.unimplemented(0x1122ca6, "Movq");
  cpu.unimplemented(0x1122ca9, "Movq");
  cpu.unimplemented(0x1122cac, "Movq");
  cpu.unimplemented(0x1122caf, "Psrlq");
  cpu.unimplemented(0x1122cb3, "Psllq");
  cpu.unimplemented(0x1122cb7, "Pand");
  cpu.unimplemented(0x1122cbe, "Por");
  cpu.unimplemented(0x1122cc1, "Psrlq");
  cpu.unimplemented(0x1122cc5, "Pand");
  cpu.unimplemented(0x1122ccc, "Psrlq");
  cpu.unimplemented(0x1122cd0, "Paddw");
  cpu.unimplemented(0x1122cd3, "Movq");
  cpu.unimplemented(0x1122cd6, "Punpcklwd");
  cpu.unimplemented(0x1122cd9, "Punpckhwd");
  cpu.unimplemented(0x1122cdc, "Movq");
  cpu.unimplemented(0x1122cdf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122ce9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122ca2; continue; }
  pc = 0x1122cef; continue;
 }
 0x1122cef => {
  cpu.unimplemented(0x1122cef, "Punpcklwd");
  cpu.unimplemented(0x1122cf2, "Punpckhwd");
  cpu.unimplemented(0x1122cf5, "Movq");
  cpu.unimplemented(0x1122cf8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122d10, "Movq");
  cpu.unimplemented(0x1122d17, "Movq");
  pc = 0x1122d1a; continue;
 }
 0x1122d1a => {
  cpu.unimplemented(0x1122d1a, "Movq");
  cpu.unimplemented(0x1122d1e, "Movq");
  cpu.unimplemented(0x1122d21, "Movq");
  cpu.unimplemented(0x1122d24, "Movq");
  cpu.unimplemented(0x1122d27, "Psrlq");
  cpu.unimplemented(0x1122d2b, "Psllq");
  cpu.unimplemented(0x1122d2f, "Pand");
  cpu.unimplemented(0x1122d36, "Por");
  cpu.unimplemented(0x1122d39, "Psrlq");
  cpu.unimplemented(0x1122d3d, "Pand");
  cpu.unimplemented(0x1122d44, "Psrlq");
  cpu.unimplemented(0x1122d48, "Paddw");
  cpu.unimplemented(0x1122d4b, "Movq");
  cpu.unimplemented(0x1122d4e, "Punpcklwd");
  cpu.unimplemented(0x1122d51, "Punpckhwd");
  cpu.unimplemented(0x1122d54, "Movq");
  cpu.unimplemented(0x1122d57, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122d61, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122d1a; continue; }
  pc = 0x1122d67; continue;
 }
 0x1122d67 => {
  cpu.unimplemented(0x1122d67, "Punpcklwd");
  cpu.unimplemented(0x1122d6a, "Punpckhwd");
  cpu.unimplemented(0x1122d6d, "Movq");
  cpu.unimplemented(0x1122d70, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1122d7f; continue;
 }
 0x1122d7f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122d87, "Movq");
  cpu.unimplemented(0x1122d8e, "Movq");
  pc = 0x1122d91; continue;
 }
 0x1122d91 => {
  cpu.unimplemented(0x1122d91, "Movq");
  cpu.unimplemented(0x1122d95, "Movq");
  cpu.unimplemented(0x1122d98, "Movq");
  cpu.unimplemented(0x1122d9b, "Movq");
  cpu.unimplemented(0x1122d9e, "Psrlq");
  cpu.unimplemented(0x1122da2, "Psllq");
  cpu.unimplemented(0x1122da6, "Pand");
  cpu.unimplemented(0x1122dad, "Por");
  cpu.unimplemented(0x1122db0, "Psrlq");
  cpu.unimplemented(0x1122db4, "Pand");
  cpu.unimplemented(0x1122dbb, "Psrlq");
  cpu.unimplemented(0x1122dbf, "Paddw");
  cpu.unimplemented(0x1122dc2, "Movq");
  cpu.unimplemented(0x1122dc5, "Punpcklwd");
  cpu.unimplemented(0x1122dc8, "Punpckhwd");
  cpu.unimplemented(0x1122dcb, "Movq");
  cpu.unimplemented(0x1122dce, "Pand");
  cpu.unimplemented(0x1122dd1, "Movq");
  cpu.unimplemented(0x1122dd8, "Movq");
  cpu.unimplemented(0x1122ddc, "Pand");
  cpu.unimplemented(0x1122ddf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122dec, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122d91; continue; }
  pc = 0x1122df2; continue;
 }
 0x1122df2 => {
  cpu.unimplemented(0x1122df2, "Punpcklwd");
  cpu.unimplemented(0x1122df5, "Punpckhwd");
  cpu.unimplemented(0x1122df8, "Movq");
  cpu.unimplemented(0x1122dfb, "Pand");
  cpu.unimplemented(0x1122dfe, "Movq");
  cpu.unimplemented(0x1122e05, "Movq");
  cpu.unimplemented(0x1122e09, "Pand");
  cpu.unimplemented(0x1122e0c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122d7f; continue; }
  pc = 0x1122e26; continue;
 }
 0x1122e26 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1122e7b; continue; }
  pc = 0x1122e3d; continue;
 }
 0x1122e3d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1122e72; continue; }
  pc = 0x1122e61; continue;
 }
 0x1122e61 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1122e72; continue;
 }
 0x1122e72 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1122e7b; continue;
 }
 0x1122e7b => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x1122e86; continue;
 }
 0x1122e86 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122e8e, "Movq");
  cpu.unimplemented(0x1122e95, "Movq");
  pc = 0x1122e98; continue;
 }
 0x1122e98 => {
  cpu.unimplemented(0x1122e98, "Movq");
  cpu.unimplemented(0x1122e9c, "Movq");
  cpu.unimplemented(0x1122e9f, "Movq");
  cpu.unimplemented(0x1122ea2, "Movq");
  cpu.unimplemented(0x1122ea5, "Psrlq");
  cpu.unimplemented(0x1122ea9, "Psllq");
  cpu.unimplemented(0x1122ead, "Pand");
  cpu.unimplemented(0x1122eb4, "Por");
  cpu.unimplemented(0x1122eb7, "Psrlq");
  cpu.unimplemented(0x1122ebb, "Pand");
  cpu.unimplemented(0x1122ec2, "Psrlq");
  cpu.unimplemented(0x1122ec6, "Paddw");
  cpu.unimplemented(0x1122ec9, "Movq");
  cpu.unimplemented(0x1122ecc, "Punpcklwd");
  cpu.unimplemented(0x1122ecf, "Punpckhwd");
  cpu.unimplemented(0x1122ed2, "Movq");
  cpu.unimplemented(0x1122ed5, "Pand");
  cpu.unimplemented(0x1122ed8, "Movq");
  cpu.unimplemented(0x1122edf, "Movq");
  cpu.unimplemented(0x1122ee3, "Pand");
  cpu.unimplemented(0x1122ee6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122ef3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122e98; continue; }
  pc = 0x1122ef9; continue;
 }
 0x1122ef9 => {
  cpu.unimplemented(0x1122ef9, "Punpcklwd");
  cpu.unimplemented(0x1122efc, "Punpckhwd");
  cpu.unimplemented(0x1122eff, "Movq");
  cpu.unimplemented(0x1122f02, "Pand");
  cpu.unimplemented(0x1122f05, "Movq");
  cpu.unimplemented(0x1122f0c, "Movq");
  cpu.unimplemented(0x1122f10, "Pand");
  cpu.unimplemented(0x1122f13, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1122e86; continue; }
  pc = 0x1122f2d; continue;
 }
 0x1122f2d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122f35, "Movq");
  cpu.unimplemented(0x1122f3c, "Movq");
  pc = 0x1122f3f; continue;
 }
 0x1122f3f => {
  cpu.unimplemented(0x1122f3f, "Movq");
  cpu.unimplemented(0x1122f43, "Movq");
  cpu.unimplemented(0x1122f46, "Movq");
  cpu.unimplemented(0x1122f49, "Movq");
  cpu.unimplemented(0x1122f4c, "Psrlq");
  cpu.unimplemented(0x1122f50, "Psllq");
  cpu.unimplemented(0x1122f54, "Pand");
  cpu.unimplemented(0x1122f5b, "Por");
  cpu.unimplemented(0x1122f5e, "Psrlq");
  cpu.unimplemented(0x1122f62, "Pand");
  cpu.unimplemented(0x1122f69, "Psrlq");
  cpu.unimplemented(0x1122f6d, "Paddw");
  cpu.unimplemented(0x1122f70, "Movq");
  cpu.unimplemented(0x1122f73, "Punpcklwd");
  cpu.unimplemented(0x1122f76, "Punpckhwd");
  cpu.unimplemented(0x1122f79, "Movq");
  cpu.unimplemented(0x1122f7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122f86, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122f3f; continue; }
  pc = 0x1122f8c; continue;
 }
 0x1122f8c => {
  cpu.unimplemented(0x1122f8c, "Punpcklwd");
  cpu.unimplemented(0x1122f8f, "Punpckhwd");
  cpu.unimplemented(0x1122f92, "Movq");
  cpu.unimplemented(0x1122f95, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1122fad, "Movq");
  cpu.unimplemented(0x1122fb4, "Movq");
  pc = 0x1122fb7; continue;
 }
 0x1122fb7 => {
  cpu.unimplemented(0x1122fb7, "Movq");
  cpu.unimplemented(0x1122fbb, "Movq");
  cpu.unimplemented(0x1122fbe, "Movq");
  cpu.unimplemented(0x1122fc1, "Movq");
  cpu.unimplemented(0x1122fc4, "Psrlq");
  cpu.unimplemented(0x1122fc8, "Psllq");
  cpu.unimplemented(0x1122fcc, "Pand");
  cpu.unimplemented(0x1122fd3, "Por");
  cpu.unimplemented(0x1122fd6, "Psrlq");
  cpu.unimplemented(0x1122fda, "Pand");
  cpu.unimplemented(0x1122fe1, "Psrlq");
  cpu.unimplemented(0x1122fe5, "Paddw");
  cpu.unimplemented(0x1122fe8, "Movq");
  cpu.unimplemented(0x1122feb, "Punpcklwd");
  cpu.unimplemented(0x1122fee, "Punpckhwd");
  cpu.unimplemented(0x1122ff1, "Movq");
  cpu.unimplemented(0x1122ff4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1122ffe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1122fb7; continue; }
  pc = 0x1123004; continue;
 }
 0x1123004 => {
  cpu.unimplemented(0x1123004, "Punpcklwd");
  cpu.unimplemented(0x1123007, "Punpckhwd");
  cpu.unimplemented(0x112300a, "Movq");
  cpu.unimplemented(0x112300d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112306c; continue; }
  pc = 0x112302e; continue;
 }
 0x112302e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1123063; continue; }
  pc = 0x1123052; continue;
 }
 0x1123052 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1123063; continue;
 }
 0x1123063 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112306c; continue;
 }
 0x112306c => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x112307a, "Movq");
  cpu.unimplemented(0x1123081, "Movq");
  pc = 0x1123084; continue;
 }
 0x1123084 => {
  cpu.unimplemented(0x1123084, "Movq");
  cpu.unimplemented(0x1123088, "Movq");
  cpu.unimplemented(0x112308b, "Movq");
  cpu.unimplemented(0x112308e, "Movq");
  cpu.unimplemented(0x1123091, "Psrlq");
  cpu.unimplemented(0x1123095, "Psllq");
  cpu.unimplemented(0x1123099, "Pand");
  cpu.unimplemented(0x11230a0, "Por");
  cpu.unimplemented(0x11230a3, "Psrlq");
  cpu.unimplemented(0x11230a7, "Pand");
  cpu.unimplemented(0x11230ae, "Psrlq");
  cpu.unimplemented(0x11230b2, "Paddw");
  cpu.unimplemented(0x11230b5, "Movq");
  cpu.unimplemented(0x11230b8, "Punpcklwd");
  cpu.unimplemented(0x11230bb, "Punpckhwd");
  cpu.unimplemented(0x11230be, "Movq");
  cpu.unimplemented(0x11230c1, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11230cb, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1123084; continue; }
  pc = 0x11230d1; continue;
 }
 0x11230d1 => {
  cpu.unimplemented(0x11230d1, "Punpcklwd");
  cpu.unimplemented(0x11230d4, "Punpckhwd");
  cpu.unimplemented(0x11230d7, "Movq");
  cpu.unimplemented(0x11230da, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x11230e9; continue;
 }
 0x11230e9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11230f1, "Movq");
  cpu.unimplemented(0x11230f8, "Movq");
  pc = 0x11230fb; continue;
 }
 0x11230fb => {
  cpu.unimplemented(0x11230fb, "Movq");
  cpu.unimplemented(0x11230ff, "Movq");
  cpu.unimplemented(0x1123102, "Movq");
  cpu.unimplemented(0x1123105, "Movq");
  cpu.unimplemented(0x1123108, "Psrlq");
  cpu.unimplemented(0x112310c, "Psllq");
  cpu.unimplemented(0x1123110, "Pand");
  cpu.unimplemented(0x1123117, "Por");
  cpu.unimplemented(0x112311a, "Psrlq");
  cpu.unimplemented(0x112311e, "Pand");
  cpu.unimplemented(0x1123125, "Psrlq");
  cpu.unimplemented(0x1123129, "Paddw");
  cpu.unimplemented(0x112312c, "Movq");
  cpu.unimplemented(0x112312f, "Punpcklwd");
  cpu.unimplemented(0x1123132, "Punpckhwd");
  cpu.unimplemented(0x1123135, "Movq");
  cpu.unimplemented(0x1123138, "Pand");
  cpu.unimplemented(0x112313b, "Movq");
  cpu.unimplemented(0x1123142, "Movq");
  cpu.unimplemented(0x1123146, "Pand");
  cpu.unimplemented(0x1123149, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x1123156, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11230fb; continue; }
  pc = 0x112315c; continue;
 }
 0x112315c => {
  cpu.unimplemented(0x112315c, "Punpcklwd");
  cpu.unimplemented(0x112315f, "Punpckhwd");
  cpu.unimplemented(0x1123162, "Movq");
  cpu.unimplemented(0x1123165, "Pand");
  cpu.unimplemented(0x1123168, "Movq");
  cpu.unimplemented(0x112316f, "Movq");
  cpu.unimplemented(0x1123173, "Pand");
  cpu.unimplemented(0x1123176, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11230e9; continue; }
  pc = 0x1123190; continue;
 }
 0x1123190 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123198, "Movq");
  cpu.unimplemented(0x112319f, "Movq");
  pc = 0x11231a2; continue;
 }
 0x11231a2 => {
  cpu.unimplemented(0x11231a2, "Movq");
  cpu.unimplemented(0x11231a6, "Movq");
  cpu.unimplemented(0x11231a9, "Movq");
  cpu.unimplemented(0x11231ac, "Movq");
  cpu.unimplemented(0x11231af, "Psrlq");
  cpu.unimplemented(0x11231b3, "Psllq");
  cpu.unimplemented(0x11231b7, "Pand");
  cpu.unimplemented(0x11231be, "Por");
  cpu.unimplemented(0x11231c1, "Psrlq");
  cpu.unimplemented(0x11231c5, "Pand");
  cpu.unimplemented(0x11231cc, "Psrlq");
  cpu.unimplemented(0x11231d0, "Paddw");
  cpu.unimplemented(0x11231d3, "Movq");
  cpu.unimplemented(0x11231d6, "Punpcklwd");
  cpu.unimplemented(0x11231d9, "Punpckhwd");
  cpu.unimplemented(0x11231dc, "Movq");
  cpu.unimplemented(0x11231df, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11231e9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11231a2; continue; }
  pc = 0x11231ef; continue;
 }
 0x11231ef => {
  cpu.unimplemented(0x11231ef, "Punpcklwd");
  cpu.unimplemented(0x11231f2, "Punpckhwd");
  cpu.unimplemented(0x11231f5, "Movq");
  cpu.unimplemented(0x11231f8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1123257; continue; }
  pc = 0x1123219; continue;
 }
 0x1123219 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112324e; continue; }
  pc = 0x112323d; continue;
 }
 0x112323d => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112324e; continue;
 }
 0x112324e => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1123257; continue;
 }
 0x1123257 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123265, "Movq");
  cpu.unimplemented(0x112326c, "Movq");
  pc = 0x112326f; continue;
 }
 0x112326f => {
  cpu.unimplemented(0x112326f, "Movq");
  cpu.unimplemented(0x1123273, "Movq");
  cpu.unimplemented(0x1123276, "Movq");
  cpu.unimplemented(0x1123279, "Movq");
  cpu.unimplemented(0x112327c, "Psrlq");
  cpu.unimplemented(0x1123280, "Psllq");
  cpu.unimplemented(0x1123284, "Pand");
  cpu.unimplemented(0x112328b, "Por");
  cpu.unimplemented(0x112328e, "Psrlq");
  cpu.unimplemented(0x1123292, "Pand");
  cpu.unimplemented(0x1123299, "Psrlq");
  cpu.unimplemented(0x112329d, "Paddw");
  cpu.unimplemented(0x11232a0, "Movq");
  cpu.unimplemented(0x11232a3, "Punpcklwd");
  cpu.unimplemented(0x11232a6, "Punpckhwd");
  cpu.unimplemented(0x11232a9, "Movq");
  cpu.unimplemented(0x11232ac, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11232b6, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x112326f; continue; }
  pc = 0x11232bc; continue;
 }
 0x11232bc => {
  cpu.unimplemented(0x11232bc, "Punpcklwd");
  cpu.unimplemented(0x11232bf, "Punpckhwd");
  cpu.unimplemented(0x11232c2, "Movq");
  cpu.unimplemented(0x11232c5, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11232dd, "Movq");
  cpu.unimplemented(0x11232e4, "Movq");
  pc = 0x11232e7; continue;
 }
 0x11232e7 => {
  cpu.unimplemented(0x11232e7, "Movq");
  cpu.unimplemented(0x11232eb, "Movq");
  cpu.unimplemented(0x11232ee, "Movq");
  cpu.unimplemented(0x11232f1, "Movq");
  cpu.unimplemented(0x11232f4, "Psrlq");
  cpu.unimplemented(0x11232f8, "Psllq");
  cpu.unimplemented(0x11232fc, "Pand");
  cpu.unimplemented(0x1123303, "Por");
  cpu.unimplemented(0x1123306, "Psrlq");
  cpu.unimplemented(0x112330a, "Pand");
  cpu.unimplemented(0x1123311, "Psrlq");
  cpu.unimplemented(0x1123315, "Paddw");
  cpu.unimplemented(0x1123318, "Movq");
  cpu.unimplemented(0x112331b, "Punpcklwd");
  cpu.unimplemented(0x112331e, "Punpckhwd");
  cpu.unimplemented(0x1123321, "Movq");
  cpu.unimplemented(0x1123324, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x112332e, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11232e7; continue; }
  pc = 0x1123334; continue;
 }
 0x1123334 => {
  cpu.unimplemented(0x1123334, "Punpcklwd");
  cpu.unimplemented(0x1123337, "Punpckhwd");
  cpu.unimplemented(0x112333a, "Movq");
  cpu.unimplemented(0x112333d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x112366b; continue; }
  pc = 0x1123357; continue;
 }
 0x1123357 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x112335c; continue;
 }
 0x112335c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123364, "Movq");
  cpu.unimplemented(0x112336b, "Movq");
  pc = 0x112336e; continue;
 }
 0x112336e => {
  cpu.unimplemented(0x112336e, "Movq");
  cpu.unimplemented(0x1123372, "Movq");
  cpu.unimplemented(0x1123375, "Movq");
  cpu.unimplemented(0x1123378, "Movq");
  cpu.unimplemented(0x112337b, "Psrlq");
  cpu.unimplemented(0x112337f, "Psllq");
  cpu.unimplemented(0x1123383, "Pand");
  cpu.unimplemented(0x112338a, "Por");
  cpu.unimplemented(0x112338d, "Psrlq");
  cpu.unimplemented(0x1123391, "Pand");
  cpu.unimplemented(0x1123398, "Psrlq");
  cpu.unimplemented(0x112339c, "Paddw");
  cpu.unimplemented(0x112339f, "Movq");
  cpu.unimplemented(0x11233a2, "Punpcklwd");
  cpu.unimplemented(0x11233a5, "Punpckhwd");
  cpu.unimplemented(0x11233a8, "Movq");
  cpu.unimplemented(0x11233ab, "Pand");
  cpu.unimplemented(0x11233ae, "Movq");
  cpu.unimplemented(0x11233b5, "Movq");
  cpu.unimplemented(0x11233b9, "Pand");
  cpu.unimplemented(0x11233bc, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11233c9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x112336e; continue; }
  pc = 0x11233cf; continue;
 }
 0x11233cf => {
  cpu.unimplemented(0x11233cf, "Punpcklwd");
  cpu.unimplemented(0x11233d2, "Punpckhwd");
  cpu.unimplemented(0x11233d5, "Movq");
  cpu.unimplemented(0x11233d8, "Pand");
  cpu.unimplemented(0x11233db, "Movq");
  cpu.unimplemented(0x11233e2, "Movq");
  cpu.unimplemented(0x11233e6, "Pand");
  cpu.unimplemented(0x11233e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112335c; continue; }
  pc = 0x1123403; continue;
 }
 0x1123403 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x112340b, "Movq");
  cpu.unimplemented(0x1123412, "Movq");
  pc = 0x1123415; continue;
 }
 0x1123415 => {
  cpu.unimplemented(0x1123415, "Movq");
  cpu.unimplemented(0x1123419, "Movq");
  cpu.unimplemented(0x112341c, "Movq");
  cpu.unimplemented(0x112341f, "Movq");
  cpu.unimplemented(0x1123422, "Psrlq");
  cpu.unimplemented(0x1123426, "Psllq");
  cpu.unimplemented(0x112342a, "Pand");
  cpu.unimplemented(0x1123431, "Por");
  cpu.unimplemented(0x1123434, "Psrlq");
  cpu.unimplemented(0x1123438, "Pand");
  cpu.unimplemented(0x112343f, "Psrlq");
  cpu.unimplemented(0x1123443, "Paddw");
  cpu.unimplemented(0x1123446, "Movq");
  cpu.unimplemented(0x1123449, "Punpcklwd");
  cpu.unimplemented(0x112344c, "Punpckhwd");
  cpu.unimplemented(0x112344f, "Movq");
  cpu.unimplemented(0x1123452, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x112345c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1123415; continue; }
  pc = 0x1123462; continue;
 }
 0x1123462 => {
  cpu.unimplemented(0x1123462, "Punpcklwd");
  cpu.unimplemented(0x1123465, "Punpckhwd");
  cpu.unimplemented(0x1123468, "Movq");
  cpu.unimplemented(0x112346b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123483, "Movq");
  cpu.unimplemented(0x112348a, "Movq");
  pc = 0x112348d; continue;
 }
 0x112348d => {
  cpu.unimplemented(0x112348d, "Movq");
  cpu.unimplemented(0x1123491, "Movq");
  cpu.unimplemented(0x1123494, "Movq");
  cpu.unimplemented(0x1123497, "Movq");
  cpu.unimplemented(0x112349a, "Psrlq");
  cpu.unimplemented(0x112349e, "Psllq");
  cpu.unimplemented(0x11234a2, "Pand");
  cpu.unimplemented(0x11234a9, "Por");
  cpu.unimplemented(0x11234ac, "Psrlq");
  cpu.unimplemented(0x11234b0, "Pand");
  cpu.unimplemented(0x11234b7, "Psrlq");
  cpu.unimplemented(0x11234bb, "Paddw");
  cpu.unimplemented(0x11234be, "Movq");
  cpu.unimplemented(0x11234c1, "Punpcklwd");
  cpu.unimplemented(0x11234c4, "Punpckhwd");
  cpu.unimplemented(0x11234c7, "Movq");
  cpu.unimplemented(0x11234ca, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11234d4, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x112348d; continue; }
  pc = 0x11234da; continue;
 }
 0x11234da => {
  cpu.unimplemented(0x11234da, "Punpcklwd");
  cpu.unimplemented(0x11234dd, "Punpckhwd");
  cpu.unimplemented(0x11234e0, "Movq");
  cpu.unimplemented(0x11234e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1123542; continue; }
  pc = 0x1123504; continue;
 }
 0x1123504 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1123539; continue; }
  pc = 0x1123528; continue;
 }
 0x1123528 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1123539; continue;
 }
 0x1123539 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1123542; continue;
 }
 0x1123542 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123550, "Movq");
  cpu.unimplemented(0x1123557, "Movq");
  pc = 0x112355a; continue;
 }
 0x112355a => {
  cpu.unimplemented(0x112355a, "Movq");
  cpu.unimplemented(0x112355e, "Movq");
  cpu.unimplemented(0x1123561, "Movq");
  cpu.unimplemented(0x1123564, "Movq");
  cpu.unimplemented(0x1123567, "Psrlq");
  cpu.unimplemented(0x112356b, "Psllq");
  cpu.unimplemented(0x112356f, "Pand");
  cpu.unimplemented(0x1123576, "Por");
  cpu.unimplemented(0x1123579, "Psrlq");
  cpu.unimplemented(0x112357d, "Pand");
  cpu.unimplemented(0x1123584, "Psrlq");
  cpu.unimplemented(0x1123588, "Paddw");
  cpu.unimplemented(0x112358b, "Movq");
  cpu.unimplemented(0x112358e, "Punpcklwd");
  cpu.unimplemented(0x1123591, "Punpckhwd");
  cpu.unimplemented(0x1123594, "Movq");
  cpu.unimplemented(0x1123597, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11235a1, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x112355a; continue; }
  pc = 0x11235a7; continue;
 }
 0x11235a7 => {
  cpu.unimplemented(0x11235a7, "Punpcklwd");
  cpu.unimplemented(0x11235aa, "Punpckhwd");
  cpu.unimplemented(0x11235ad, "Movq");
  cpu.unimplemented(0x11235b0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x11235bf; continue;
 }
 0x11235bf => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x11235c7, "Movq");
  cpu.unimplemented(0x11235ce, "Movq");
  pc = 0x11235d1; continue;
 }
 0x11235d1 => {
  cpu.unimplemented(0x11235d1, "Movq");
  cpu.unimplemented(0x11235d5, "Movq");
  cpu.unimplemented(0x11235d8, "Movq");
  cpu.unimplemented(0x11235db, "Movq");
  cpu.unimplemented(0x11235de, "Psrlq");
  cpu.unimplemented(0x11235e2, "Psllq");
  cpu.unimplemented(0x11235e6, "Pand");
  cpu.unimplemented(0x11235ed, "Por");
  cpu.unimplemented(0x11235f0, "Psrlq");
  cpu.unimplemented(0x11235f4, "Pand");
  cpu.unimplemented(0x11235fb, "Psrlq");
  cpu.unimplemented(0x11235ff, "Paddw");
  cpu.unimplemented(0x1123602, "Movq");
  cpu.unimplemented(0x1123605, "Punpcklwd");
  cpu.unimplemented(0x1123608, "Punpckhwd");
  cpu.unimplemented(0x112360b, "Movq");
  cpu.unimplemented(0x112360e, "Pand");
  cpu.unimplemented(0x1123611, "Movq");
  cpu.unimplemented(0x1123618, "Movq");
  cpu.unimplemented(0x112361c, "Pand");
  cpu.unimplemented(0x112361f, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x112362c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x11235d1; continue; }
  pc = 0x1123632; continue;
 }
 0x1123632 => {
  cpu.unimplemented(0x1123632, "Punpcklwd");
  cpu.unimplemented(0x1123635, "Punpckhwd");
  cpu.unimplemented(0x1123638, "Movq");
  cpu.unimplemented(0x112363b, "Pand");
  cpu.unimplemented(0x112363e, "Movq");
  cpu.unimplemented(0x1123645, "Movq");
  cpu.unimplemented(0x1123649, "Pand");
  cpu.unimplemented(0x112364c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11235bf; continue; }
  pc = 0x1123666; continue;
 }
 0x1123666 => {
  pc = 0x1123717; continue;
 }
 0x112366b => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x1123670; continue;
 }
 0x1123670 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x1123678, "Movq");
  cpu.unimplemented(0x112367f, "Movq");
  pc = 0x1123682; continue;
 }
 0x1123682 => {
  cpu.unimplemented(0x1123682, "Movq");
  cpu.unimplemented(0x1123686, "Movq");
  cpu.unimplemented(0x1123689, "Movq");
  cpu.unimplemented(0x112368c, "Movq");
  cpu.unimplemented(0x112368f, "Psrlq");
  cpu.unimplemented(0x1123693, "Psllq");
  cpu.unimplemented(0x1123697, "Pand");
  cpu.unimplemented(0x112369e, "Por");
  cpu.unimplemented(0x11236a1, "Psrlq");
  cpu.unimplemented(0x11236a5, "Pand");
  cpu.unimplemented(0x11236ac, "Psrlq");
  cpu.unimplemented(0x11236b0, "Paddw");
  cpu.unimplemented(0x11236b3, "Movq");
  cpu.unimplemented(0x11236b6, "Punpcklwd");
  cpu.unimplemented(0x11236b9, "Punpckhwd");
  cpu.unimplemented(0x11236bc, "Movq");
  cpu.unimplemented(0x11236bf, "Pand");
  cpu.unimplemented(0x11236c2, "Movq");
  cpu.unimplemented(0x11236c9, "Movq");
  cpu.unimplemented(0x11236cd, "Pand");
  cpu.unimplemented(0x11236d0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x11236dd, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x1123682; continue; }
  pc = 0x11236e3; continue;
 }
 0x11236e3 => {
  cpu.unimplemented(0x11236e3, "Punpcklwd");
  cpu.unimplemented(0x11236e6, "Punpckhwd");
  cpu.unimplemented(0x11236e9, "Movq");
  cpu.unimplemented(0x11236ec, "Pand");
  cpu.unimplemented(0x11236ef, "Movq");
  cpu.unimplemented(0x11236f6, "Movq");
  cpu.unimplemented(0x11236fa, "Pand");
  cpu.unimplemented(0x11236fd, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123670; continue; }
  pc = 0x1123717; continue;
 }
 0x1123717 => {
  cpu.unimplemented(0x1123717, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112371b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112371b); }
 let mut pc: u32 = 0x112371b;
 loop { match pc {
 0x112371b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123720); }
 let mut pc: u32 = 0x1123720;
 loop { match pc {
 0x1123720 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123725); }
 let mut pc: u32 = 0x1123725;
 loop { match pc {
 0x1123725 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1251b4cu32); cpu.ecx = v; }
  pc = 0x1123734; continue;
 }
 0x1123734 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x1123734; continue; }
  pc = 0x1123757; continue;
 }
 0x1123757 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123759); }
 let mut pc: u32 = 0x1123759;
 loop { match pc {
 0x1123759 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x112379c; continue;
 }
 0x112379c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112379c; continue; }
  pc = 0x11237ae; continue;
 }
 0x11237ae => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_011237b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11237b1); }
 let mut pc: u32 = 0x11237b1;
 loop { match pc {
 0x11237b1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x11237f4; continue;
 }
 0x11237f4 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11237f4; continue; }
  pc = 0x1123805; continue;
 }
 0x1123805 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123808); }
 let mut pc: u32 = 0x1123808;
 loop { match pc {
 0x1123808 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 1u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1123823; continue; }
  pc = 0x112381d; continue;
 }
 0x112381d => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  pc = 0x1123823; continue;
 }
 0x1123823 => {
  { let v: u32 = cpu.r32(0x1251b68u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x112382b; continue;
 }
 0x112382b => {
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x1123852; continue;
 }
 0x1123836 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x1123852; continue;
 }
 0x1123852 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123836; continue; }
  pc = 0x1123855; continue;
 }
 0x1123855 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112382b; continue; }
  pc = 0x112385a; continue;
 }
 0x112385a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112385c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112385c); }
 let mut pc: u32 = 0x112385c;
 loop { match pc {
 0x112385c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x112387b; continue; }
  pc = 0x1123878; continue;
 }
 0x1123878 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x112387b; continue;
 }
 0x112387b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x11238a9; continue;
 }
 0x11238a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x11238bd; continue;
 }
 0x11238ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x11238bd; continue;
 }
 0x11238bd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11238ae; continue; }
  pc = 0x11238c0; continue;
 }
 0x11238c0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11238a9; continue; }
  pc = 0x11238c9; continue;
 }
 0x11238c9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_011238cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11238cc); }
 let mut pc: u32 = 0x11238cc;
 loop { match pc {
 0x11238cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x11238eb; continue; }
  pc = 0x11238e8; continue;
 }
 0x11238e8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x11238eb; continue;
 }
 0x11238eb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b6cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x1123919; continue;
 }
 0x1123919 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x1123921; continue;
 }
 0x112391e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x1123921; continue;
 }
 0x1123921 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112391e; continue; }
  pc = 0x1123924; continue;
 }
 0x1123924 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123919; continue; }
  pc = 0x112392d; continue;
 }
 0x112392d => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123930); }
 let mut pc: u32 = 0x1123930;
 loop { match pc {
 0x1123930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300034u32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x130001cu32, v); }
  pc = 0x112394d; continue;
 }
 0x112394d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123960; continue; }
  pc = 0x112395a; continue;
 }
 0x112395a => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123960; continue;
 }
 0x1123960 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x11239a4; continue;
 }
 0x1123973 => {
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
  pc = 0x11239a4; continue;
 }
 0x11239a4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123973; continue; }
  pc = 0x11239ac; continue;
 }
 0x11239ac => {
  { let v: u32 = cpu.dec32(cpu.r32(0x130001cu32)); cpu.w32(0x130001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112394d; continue; }
  pc = 0x11239b4; continue;
 }
 0x11239b4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_011239b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x11239b6); }
 let mut pc: u32 = 0x11239b6;
 loop { match pc {
 0x11239b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x500u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300034u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300024u32, v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300028u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x130002cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1300030u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x130001cu32, v); }
  pc = 0x1123a26; continue;
 }
 0x1123a26 => {
  { let v: u32 = cpu.r32(0x130002cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123a3a; continue; }
  pc = 0x1123a34; continue;
 }
 0x1123a34 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123a3a; continue;
 }
 0x1123a3a => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1123a7e; continue;
 }
 0x1123a4d => {
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
  pc = 0x1123a7e; continue;
 }
 0x1123a7e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123a4d; continue; }
  pc = 0x1123a86; continue;
 }
 0x1123a86 => {
  { let v: u32 = cpu.r32(0x1300030u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123a9a; continue; }
  pc = 0x1123a94; continue;
 }
 0x1123a94 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123a9a; continue;
 }
 0x1123a9a => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebx = v; }
  pc = 0x1123ab9; continue;
 }
 0x1123aa2 => {
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
  pc = 0x1123ab9; continue;
 }
 0x1123ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123aa2; continue; }
  pc = 0x1123abc; continue;
 }
 0x1123abc => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1300024u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x1300028u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x1300028u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x130001cu32)); cpu.w32(0x130001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123a26; continue; }
  pc = 0x1123ada; continue;
 }
 0x1123ada => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1123ade); }
 let mut pc: u32 = 0x1123ade;
 loop { match pc {
 0x1123ade => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1123b15; continue;
 }
 0x1123b15 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1123b2c; continue;
 }
 0x1123b22 => {
  { cpu.lods(1, false); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1123b2c; continue;
 }
 0x1123b2c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123b22; continue; }
  pc = 0x1123b2f; continue;
 }
 0x1123b2f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123b15; continue; }
  pc = 0x1123b3a; continue;
 }
 0x1123b3a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01123b3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1123b3d); } r_01123b3d(cpu, 0x1123b3d); }
pub fn f_01123dff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1123dff); } r_01123b3d(cpu, 0x1123dff); }
fn r_01123b3d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1123b3d => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1123b8f; continue; }
  pc = 0x1123b51; continue;
 }
 0x1123b51 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1123b86; continue; }
  pc = 0x1123b75; continue;
 }
 0x1123b75 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1123b86; continue;
 }
 0x1123b86 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1123b8f; continue;
 }
 0x1123b8f => {
  { let v: u32 = cpu.edi; cpu.w32(0x1300020u32, v); }
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300034u32, v); }
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1123bc9; continue; }
  pc = 0x1123bb8; continue;
 }
 0x1123bb8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x6400u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6400u32); cpu.edx = v; }
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x1123bc9; continue;
 }
 0x1123bc9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123bdd; continue; }
  pc = 0x1123bd7; continue;
 }
 0x1123bd7 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123bdd; continue;
 }
 0x1123bdd => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1123c7c; continue;
 }
 0x1123bf3 => {
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
  pc = 0x1123c7c; continue;
 }
 0x1123c7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123bf3; continue; }
  pc = 0x1123c88; continue;
 }
 0x1123c88 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123bc9; continue; }
  pc = 0x1123c9c; continue;
 }
 0x1123c9c => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123cb0; continue; }
  pc = 0x1123caa; continue;
 }
 0x1123caa => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123cb0; continue;
 }
 0x1123cb0 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1123d15; continue;
 }
 0x1123cc8 => {
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
  pc = 0x1123d15; continue;
 }
 0x1123d15 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123cc8; continue; }
  pc = 0x1123d1d; continue;
 }
 0x1123d1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1123d74; continue; }
  pc = 0x1123d36; continue;
 }
 0x1123d36 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1123d6b; continue; }
  pc = 0x1123d5a; continue;
 }
 0x1123d5a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1123d6b; continue;
 }
 0x1123d6b => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1123d74; continue;
 }
 0x1123d74 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123d8f; continue; }
  pc = 0x1123d89; continue;
 }
 0x1123d89 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123d8f; continue;
 }
 0x1123d8f => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1123df4; continue;
 }
 0x1123da7 => {
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
  pc = 0x1123df4; continue;
 }
 0x1123df4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123da7; continue; }
  pc = 0x1123dfc; continue;
 }
 0x1123dfc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123e17; continue; }
  pc = 0x1123e11; continue;
 }
 0x1123dff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0xa0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xa0u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123e17; continue; }
  pc = 0x1123e11; continue;
 }
 0x1123e11 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123e17; continue;
 }
 0x1123e17 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1123e7c; continue;
 }
 0x1123e2f => {
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
  pc = 0x1123e7c; continue;
 }
 0x1123e7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123e2f; continue; }
  pc = 0x1123e84; continue;
 }
 0x1123e84 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x1123e8a; continue;
 }
 0x1123e8a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123e9e; continue; }
  pc = 0x1123e98; continue;
 }
 0x1123e98 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123e9e; continue;
 }
 0x1123e9e => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1123f3d; continue;
 }
 0x1123eb4 => {
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
  pc = 0x1123f3d; continue;
 }
 0x1123f3d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123eb4; continue; }
  pc = 0x1123f49; continue;
 }
 0x1123f49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1123e8a; continue; }
  pc = 0x1123f5d; continue;
 }
 0x1123f5d => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1123f71; continue; }
  pc = 0x1123f6b; continue;
 }
 0x1123f6b => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1123f71; continue;
 }
 0x1123f71 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1123fd6; continue;
 }
 0x1123f89 => {
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
  pc = 0x1123fd6; continue;
 }
 0x1123fd6 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1123f89; continue; }
  pc = 0x1123fde; continue;
 }
 0x1123fde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1124035; continue; }
  pc = 0x1123ff7; continue;
 }
 0x1123ff7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112402c; continue; }
  pc = 0x112401b; continue;
 }
 0x112401b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112402c; continue;
 }
 0x112402c => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1124035; continue;
 }
 0x1124035 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124050; continue; }
  pc = 0x112404a; continue;
 }
 0x112404a => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124050; continue;
 }
 0x1124050 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x11240b5; continue;
 }
 0x1124068 => {
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
  pc = 0x11240b5; continue;
 }
 0x11240b5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124068; continue; }
  pc = 0x11240bd; continue;
 }
 0x11240bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11240d8; continue; }
  pc = 0x11240d2; continue;
 }
 0x11240d2 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11240d8; continue;
 }
 0x11240d8 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x112413d; continue;
 }
 0x11240f0 => {
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
  pc = 0x112413d; continue;
 }
 0x112413d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11240f0; continue; }
  pc = 0x1124145; continue;
 }
 0x1124145 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x112414b; continue;
 }
 0x112414b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x112415f; continue; }
  pc = 0x1124159; continue;
 }
 0x1124159 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x112415f; continue;
 }
 0x112415f => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x11241fe; continue;
 }
 0x1124175 => {
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
  pc = 0x11241fe; continue;
 }
 0x11241fe => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124175; continue; }
  pc = 0x112420a; continue;
 }
 0x112420a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112414b; continue; }
  pc = 0x112421e; continue;
 }
 0x112421e => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124232; continue; }
  pc = 0x112422c; continue;
 }
 0x112422c => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124232; continue;
 }
 0x1124232 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1124297; continue;
 }
 0x112424a => {
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
  pc = 0x1124297; continue;
 }
 0x1124297 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x112424a; continue; }
  pc = 0x112429f; continue;
 }
 0x112429f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11242ba; continue; }
  pc = 0x11242b4; continue;
 }
 0x11242b4 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11242ba; continue;
 }
 0x11242ba => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x112431f; continue;
 }
 0x11242d2 => {
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
  pc = 0x112431f; continue;
 }
 0x112431f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11242d2; continue; }
  pc = 0x1124327; continue;
 }
 0x1124327 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112437e; continue; }
  pc = 0x1124340; continue;
 }
 0x1124340 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124375; continue; }
  pc = 0x1124364; continue;
 }
 0x1124364 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124375; continue;
 }
 0x1124375 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112437e; continue;
 }
 0x112437e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x11247b5; continue; }
  pc = 0x112438f; continue;
 }
 0x112438f => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11243a9; continue; }
  pc = 0x11243a3; continue;
 }
 0x11243a3 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11243a9; continue;
 }
 0x11243a9 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x112440e; continue;
 }
 0x11243c1 => {
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
  pc = 0x112440e; continue;
 }
 0x112440e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11243c1; continue; }
  pc = 0x1124416; continue;
 }
 0x1124416 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x112441c; continue;
 }
 0x112441c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124430; continue; }
  pc = 0x112442a; continue;
 }
 0x112442a => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124430; continue;
 }
 0x1124430 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x11244cf; continue;
 }
 0x1124446 => {
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
  pc = 0x11244cf; continue;
 }
 0x11244cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124446; continue; }
  pc = 0x11244db; continue;
 }
 0x11244db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112441c; continue; }
  pc = 0x11244ef; continue;
 }
 0x11244ef => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124503; continue; }
  pc = 0x11244fd; continue;
 }
 0x11244fd => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124503; continue;
 }
 0x1124503 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1124568; continue;
 }
 0x112451b => {
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
  pc = 0x1124568; continue;
 }
 0x1124568 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x112451b; continue; }
  pc = 0x1124570; continue;
 }
 0x1124570 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x112458b; continue; }
  pc = 0x1124585; continue;
 }
 0x1124585 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x112458b; continue;
 }
 0x112458b => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x11245f0; continue;
 }
 0x11245a3 => {
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
  pc = 0x11245f0; continue;
 }
 0x11245f0 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11245a3; continue; }
  pc = 0x11245f8; continue;
 }
 0x11245f8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112464f; continue; }
  pc = 0x1124611; continue;
 }
 0x1124611 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124646; continue; }
  pc = 0x1124635; continue;
 }
 0x1124635 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124646; continue;
 }
 0x1124646 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112464f; continue;
 }
 0x112464f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x112466a; continue; }
  pc = 0x1124664; continue;
 }
 0x1124664 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x112466a; continue;
 }
 0x112466a => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x11246cf; continue;
 }
 0x1124682 => {
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
  pc = 0x11246cf; continue;
 }
 0x11246cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124682; continue; }
  pc = 0x11246d7; continue;
 }
 0x11246d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x11246dd; continue;
 }
 0x11246dd => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11246f1; continue; }
  pc = 0x11246eb; continue;
 }
 0x11246eb => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11246f1; continue;
 }
 0x11246f1 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124790; continue;
 }
 0x1124707 => {
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
  pc = 0x1124790; continue;
 }
 0x1124790 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124707; continue; }
  pc = 0x112479c; continue;
 }
 0x112479c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11246dd; continue; }
  pc = 0x11247b0; continue;
 }
 0x11247b0 => {
  pc = 0x1124915; continue;
 }
 0x11247b5 => {
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11247cf; continue; }
  pc = 0x11247c9; continue;
 }
 0x11247c9 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11247cf; continue;
 }
 0x11247cf => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x1124834; continue;
 }
 0x11247e7 => {
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
  pc = 0x1124834; continue;
 }
 0x1124834 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11247e7; continue; }
  pc = 0x112483c; continue;
 }
 0x112483c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2eu32; cpu.ebp = v; }
  pc = 0x1124842; continue;
 }
 0x1124842 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124856; continue; }
  pc = 0x1124850; continue;
 }
 0x1124850 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124856; continue;
 }
 0x1124856 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x11248f5; continue;
 }
 0x112486c => {
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
  pc = 0x11248f5; continue;
 }
 0x11248f5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x112486c; continue; }
  pc = 0x1124901; continue;
 }
 0x1124901 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1124842; continue; }
  pc = 0x1124915; continue;
 }
 0x1124915 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01124918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1124918); }
 let mut pc: u32 = 0x1124918;
 loop { match pc {
 0x1124918 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x1251100u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300034u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1124947; continue; }
  pc = 0x1124944; continue;
 }
 0x1124944 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x1124947; continue;
 }
 0x1124947 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11249b6; continue; }
  pc = 0x1124978; continue;
 }
 0x1124978 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11249ad; continue; }
  pc = 0x112499c; continue;
 }
 0x112499c => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11249ad; continue;
 }
 0x11249ad => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11249b6; continue;
 }
 0x11249b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x11249ee; continue;
 }
 0x11249ee => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x1124a85; continue; }
  pc = 0x11249fa; continue;
 }
 0x11249fa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124a0d; continue; }
  pc = 0x1124a07; continue;
 }
 0x1124a07 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124a0d; continue;
 }
 0x1124a0d => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124a64; continue;
 }
 0x1124a1f => {
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
  pc = 0x1124a64; continue;
 }
 0x1124a64 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124a1f; continue; }
  pc = 0x1124a6c; continue;
 }
 0x1124a6c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11249ee; continue; }
  pc = 0x1124a80; continue;
 }
 0x1124a80 => {
  pc = 0x1125004; continue;
 }
 0x1124a85 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x1124c2b; continue; }
  pc = 0x1124a96; continue;
 }
 0x1124a96 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124aa8; continue; }
  pc = 0x1124aa2; continue;
 }
 0x1124aa2 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124aa8; continue;
 }
 0x1124aa8 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124ae2; continue;
 }
 0x1124aba => {
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
  pc = 0x1124ae2; continue;
 }
 0x1124ae2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124aba; continue; }
  pc = 0x1124aea; continue;
 }
 0x1124aea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1124b43; continue; }
  pc = 0x1124b05; continue;
 }
 0x1124b05 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124b3a; continue; }
  pc = 0x1124b29; continue;
 }
 0x1124b29 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124b3a; continue;
 }
 0x1124b3a => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1124b43; continue;
 }
 0x1124b43 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124b67; continue; }
  pc = 0x1124b61; continue;
 }
 0x1124b61 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124b67; continue;
 }
 0x1124b67 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124ba1; continue;
 }
 0x1124b79 => {
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
  pc = 0x1124ba1; continue;
 }
 0x1124ba1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124b79; continue; }
  pc = 0x1124ba9; continue;
 }
 0x1124ba9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124bca; continue; }
  pc = 0x1124bc4; continue;
 }
 0x1124bc4 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124bca; continue;
 }
 0x1124bca => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124c04; continue;
 }
 0x1124bdc => {
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
  pc = 0x1124c04; continue;
 }
 0x1124c04 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124bdc; continue; }
  pc = 0x1124c0c; continue;
 }
 0x1124c0c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11249ee; continue; }
  pc = 0x1124c26; continue;
 }
 0x1124c26 => {
  pc = 0x1125004; continue;
 }
 0x1124c2b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x1124ed1; continue; }
  pc = 0x1124c39; continue;
 }
 0x1124c39 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x1124ddf; continue; }
  pc = 0x1124c42; continue;
 }
 0x1124c42 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124c55; continue; }
  pc = 0x1124c4f; continue;
 }
 0x1124c4f => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124c55; continue;
 }
 0x1124c55 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124c8f; continue;
 }
 0x1124c67 => {
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
  pc = 0x1124c8f; continue;
 }
 0x1124c8f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124c67; continue; }
  pc = 0x1124c97; continue;
 }
 0x1124c97 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124cbf; continue; }
  pc = 0x1124cb9; continue;
 }
 0x1124cb9 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124cbf; continue;
 }
 0x1124cbf => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124cf9; continue;
 }
 0x1124cd1 => {
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
  pc = 0x1124cf9; continue;
 }
 0x1124cf9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124cd1; continue; }
  pc = 0x1124d01; continue;
 }
 0x1124d01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1124d5a; continue; }
  pc = 0x1124d1c; continue;
 }
 0x1124d1c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124d51; continue; }
  pc = 0x1124d40; continue;
 }
 0x1124d40 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124d51; continue;
 }
 0x1124d51 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1124d5a; continue;
 }
 0x1124d5a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124d7e; continue; }
  pc = 0x1124d78; continue;
 }
 0x1124d78 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124d7e; continue;
 }
 0x1124d7e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124db8; continue;
 }
 0x1124d90 => {
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
  pc = 0x1124db8; continue;
 }
 0x1124db8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124d90; continue; }
  pc = 0x1124dc0; continue;
 }
 0x1124dc0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11249ee; continue; }
  pc = 0x1124dda; continue;
 }
 0x1124dda => {
  pc = 0x1125004; continue;
 }
 0x1124ddf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124df2; continue; }
  pc = 0x1124dec; continue;
 }
 0x1124dec => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124df2; continue;
 }
 0x1124df2 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124e49; continue;
 }
 0x1124e04 => {
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
  pc = 0x1124e49; continue;
 }
 0x1124e49 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124e04; continue; }
  pc = 0x1124e51; continue;
 }
 0x1124e51 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1124eaa; continue; }
  pc = 0x1124e6c; continue;
 }
 0x1124e6c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124ea1; continue; }
  pc = 0x1124e90; continue;
 }
 0x1124e90 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124ea1; continue;
 }
 0x1124ea1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1124eaa; continue;
 }
 0x1124eaa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11249ee; continue; }
  pc = 0x1124ecc; continue;
 }
 0x1124ecc => {
  pc = 0x1125004; continue;
 }
 0x1124ed1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124ee4; continue; }
  pc = 0x1124ede; continue;
 }
 0x1124ede => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124ee4; continue;
 }
 0x1124ee4 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124f1e; continue;
 }
 0x1124ef6 => {
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
  pc = 0x1124f1e; continue;
 }
 0x1124f1e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124ef6; continue; }
  pc = 0x1124f26; continue;
 }
 0x1124f26 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1124f85; continue; }
  pc = 0x1124f47; continue;
 }
 0x1124f47 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1124f7c; continue; }
  pc = 0x1124f6b; continue;
 }
 0x1124f6b => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1124f7c; continue;
 }
 0x1124f7c => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1124f85; continue;
 }
 0x1124f85 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1124fae; continue; }
  pc = 0x1124fa8; continue;
 }
 0x1124fa8 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1124fae; continue;
 }
 0x1124fae => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1300018u32, v); }
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  pc = 0x1124fe8; continue;
 }
 0x1124fc0 => {
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
  pc = 0x1124fe8; continue;
 }
 0x1124fe8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1124fc0; continue; }
  pc = 0x1124ff0; continue;
 }
 0x1124ff0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11249ee; continue; }
  pc = 0x1125004; continue;
 }
 0x1125004 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125007); }
 let mut pc: u32 = 0x1125007;
 loop { match pc {
 0x1125007 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1251b20u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x1125026; continue; }
  pc = 0x1125023; continue;
 }
 0x1125023 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x1125026; continue;
 }
 0x1125026 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1251b6cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x1251b48u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x1251b60u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112508e; continue; }
  pc = 0x1125050; continue;
 }
 0x1125050 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125085; continue; }
  pc = 0x1125074; continue;
 }
 0x1125074 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125085; continue;
 }
 0x1125085 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112508e; continue;
 }
 0x112508e => {
  { let v: u32 = cpu.r32(0x1251b48u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x1251b24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x11250c8; continue;
 }
 0x11250c8 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x1251b64u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x11250fb; continue; }
  pc = 0x11250d0; continue;
 }
 0x11250d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11250e9; continue;
 }
 0x11250de => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x11250e9; continue;
 }
 0x11250e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250de; continue; }
  pc = 0x11250ec; continue;
 }
 0x11250ec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250c8; continue; }
  pc = 0x11250f6; continue;
 }
 0x11250f6 => {
  pc = 0x1125427; continue;
 }
 0x11250fb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b64u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x11251e8; continue; }
  pc = 0x112510c; continue;
 }
 0x112510c => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1125124; continue;
 }
 0x1125119 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1125124; continue;
 }
 0x1125124 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125119; continue; }
  pc = 0x1125127; continue;
 }
 0x1125127 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112517f; continue; }
  pc = 0x1125141; continue;
 }
 0x1125141 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125176; continue; }
  pc = 0x1125165; continue;
 }
 0x1125165 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125176; continue;
 }
 0x1125176 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112517f; continue;
 }
 0x112517f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b64u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11251a9; continue;
 }
 0x112519e => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x11251a9; continue;
 }
 0x11251a9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112519e; continue; }
  pc = 0x11251ac; continue;
 }
 0x11251ac => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11251cc; continue;
 }
 0x11251c1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x11251cc; continue;
 }
 0x11251cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11251c1; continue; }
  pc = 0x11251cf; continue;
 }
 0x11251cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250c8; continue; }
  pc = 0x11251e3; continue;
 }
 0x11251e3 => {
  pc = 0x1125427; continue;
 }
 0x11251e8 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x1125373; continue; }
  pc = 0x11251f6; continue;
 }
 0x11251f6 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x11252e3; continue; }
  pc = 0x11251ff; continue;
 }
 0x11251ff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1125218; continue;
 }
 0x112520d => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1125218; continue;
 }
 0x1125218 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112520d; continue; }
  pc = 0x112521b; continue;
 }
 0x112521b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x1251b24u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1125242; continue;
 }
 0x1125237 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1125242; continue;
 }
 0x1125242 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125237; continue; }
  pc = 0x1125245; continue;
 }
 0x1125245 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112529d; continue; }
  pc = 0x112525f; continue;
 }
 0x112525f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125294; continue; }
  pc = 0x1125283; continue;
 }
 0x1125283 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125294; continue;
 }
 0x1125294 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112529d; continue;
 }
 0x112529d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1251b24u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11252c7; continue;
 }
 0x11252bc => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x11252c7; continue;
 }
 0x11252c7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11252bc; continue; }
  pc = 0x11252ca; continue;
 }
 0x11252ca => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250c8; continue; }
  pc = 0x11252de; continue;
 }
 0x11252de => {
  pc = 0x1125427; continue;
 }
 0x11252e3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x11252fc; continue;
 }
 0x11252f1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x11252fc; continue;
 }
 0x11252fc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x11252f1; continue; }
  pc = 0x11252ff; continue;
 }
 0x11252ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1125357; continue; }
  pc = 0x1125319; continue;
 }
 0x1125319 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x112534e; continue; }
  pc = 0x112533d; continue;
 }
 0x112533d => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x112534e; continue;
 }
 0x112534e => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1125357; continue;
 }
 0x1125357 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250c8; continue; }
  pc = 0x112536e; continue;
 }
 0x112536e => {
  pc = 0x1125427; continue;
 }
 0x1125373 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x112538c; continue;
 }
 0x1125381 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x112538c; continue;
 }
 0x112538c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125381; continue; }
  pc = 0x112538f; continue;
 }
 0x112538f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11253ea; continue; }
  pc = 0x11253ac; continue;
 }
 0x11253ac => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11253e1; continue; }
  pc = 0x11253d0; continue;
 }
 0x11253d0 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11253e1; continue;
 }
 0x11253e1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11253ea; continue;
 }
 0x11253ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1251b24u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x1251b48u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1251b80u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x1125416; continue;
 }
 0x112540b => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x1125416; continue;
 }
 0x1125416 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x112540b; continue; }
  pc = 0x1125419; continue;
 }
 0x1125419 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x11250c8; continue; }
  pc = 0x1125427; continue;
 }
 0x1125427 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0112542a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x112542a); }
 let mut pc: u32 = 0x112542a;
 loop { match pc {
 0x112542a => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112547c; continue; }
  pc = 0x112543e; continue;
 }
 0x112543e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125473; continue; }
  pc = 0x1125462; continue;
 }
 0x1125462 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125473; continue;
 }
 0x1125473 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112547c; continue;
 }
 0x112547c => {
  { let v: u32 = cpu.edi; cpu.w32(0x1300020u32, v); }
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300034u32, v); }
  { let v: u32 = 0x66u32; cpu.ebp = v; }
  pc = 0x1125499; continue;
 }
 0x1125499 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11254ad; continue; }
  pc = 0x11254a7; continue;
 }
 0x11254a7 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11254ad; continue;
 }
 0x11254ad => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x11254f1; continue;
 }
 0x11254c0 => {
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
  pc = 0x11254f1; continue;
 }
 0x11254f1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11254c0; continue; }
  pc = 0x11254f9; continue;
 }
 0x11254f9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125499; continue; }
  pc = 0x11254fd; continue;
 }
 0x11254fd => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125511; continue; }
  pc = 0x112550b; continue;
 }
 0x112550b => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125511; continue;
 }
 0x1125511 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125555; continue;
 }
 0x1125524 => {
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
  pc = 0x1125555; continue;
 }
 0x1125555 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125524; continue; }
  pc = 0x112555d; continue;
 }
 0x112555d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11255b4; continue; }
  pc = 0x1125576; continue;
 }
 0x1125576 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11255ab; continue; }
  pc = 0x112559a; continue;
 }
 0x112559a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11255ab; continue;
 }
 0x11255ab => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11255b4; continue;
 }
 0x11255b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11255cf; continue; }
  pc = 0x11255c9; continue;
 }
 0x11255c9 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11255cf; continue;
 }
 0x11255cf => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125613; continue;
 }
 0x11255e2 => {
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
  pc = 0x1125613; continue;
 }
 0x1125613 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11255e2; continue; }
  pc = 0x112561b; continue;
 }
 0x112561b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125630; continue; }
  pc = 0x112562a; continue;
 }
 0x112562a => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125630; continue;
 }
 0x1125630 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125674; continue;
 }
 0x1125643 => {
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
  pc = 0x1125674; continue;
 }
 0x1125674 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125643; continue; }
  pc = 0x112567c; continue;
 }
 0x112567c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x1125682; continue;
 }
 0x1125682 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125696; continue; }
  pc = 0x1125690; continue;
 }
 0x1125690 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125696; continue;
 }
 0x1125696 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x11256da; continue;
 }
 0x11256a9 => {
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
  pc = 0x11256da; continue;
 }
 0x11256da => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11256a9; continue; }
  pc = 0x11256e2; continue;
 }
 0x11256e2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125682; continue; }
  pc = 0x11256e6; continue;
 }
 0x11256e6 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11256fa; continue; }
  pc = 0x11256f4; continue;
 }
 0x11256f4 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11256fa; continue;
 }
 0x11256fa => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x112573e; continue;
 }
 0x112570d => {
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
  pc = 0x112573e; continue;
 }
 0x112573e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x112570d; continue; }
  pc = 0x1125746; continue;
 }
 0x1125746 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x112579d; continue; }
  pc = 0x112575f; continue;
 }
 0x112575f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125794; continue; }
  pc = 0x1125783; continue;
 }
 0x1125783 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125794; continue;
 }
 0x1125794 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x112579d; continue;
 }
 0x112579d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11257b8; continue; }
  pc = 0x11257b2; continue;
 }
 0x11257b2 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11257b8; continue;
 }
 0x11257b8 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x11257fc; continue;
 }
 0x11257cb => {
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
  pc = 0x11257fc; continue;
 }
 0x11257fc => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11257cb; continue; }
  pc = 0x1125804; continue;
 }
 0x1125804 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125819; continue; }
  pc = 0x1125813; continue;
 }
 0x1125813 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125819; continue;
 }
 0x1125819 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x112585d; continue;
 }
 0x112582c => {
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
  pc = 0x112585d; continue;
 }
 0x112585d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x112582c; continue; }
  pc = 0x1125865; continue;
 }
 0x1125865 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x112586b; continue;
 }
 0x112586b => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x112587f; continue; }
  pc = 0x1125879; continue;
 }
 0x1125879 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x112587f; continue;
 }
 0x112587f => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x11258c3; continue;
 }
 0x1125892 => {
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
  pc = 0x11258c3; continue;
 }
 0x11258c3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125892; continue; }
  pc = 0x11258cb; continue;
 }
 0x11258cb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x112586b; continue; }
  pc = 0x11258cf; continue;
 }
 0x11258cf => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x11258e3; continue; }
  pc = 0x11258dd; continue;
 }
 0x11258dd => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x11258e3; continue;
 }
 0x11258e3 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125927; continue;
 }
 0x11258f6 => {
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
  pc = 0x1125927; continue;
 }
 0x1125927 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x11258f6; continue; }
  pc = 0x112592f; continue;
 }
 0x112592f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125944; continue; }
  pc = 0x112593e; continue;
 }
 0x112593e => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125944; continue;
 }
 0x1125944 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125988; continue;
 }
 0x1125957 => {
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
  pc = 0x1125988; continue;
 }
 0x1125988 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125957; continue; }
  pc = 0x1125990; continue;
 }
 0x1125990 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x11259e7; continue; }
  pc = 0x11259a9; continue;
 }
 0x11259a9 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x11259de; continue; }
  pc = 0x11259cd; continue;
 }
 0x11259cd => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x11259de; continue;
 }
 0x11259de => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x11259e7; continue;
 }
 0x11259e7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125a02; continue; }
  pc = 0x11259fc; continue;
 }
 0x11259fc => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125a02; continue;
 }
 0x1125a02 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125a46; continue;
 }
 0x1125a15 => {
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
  pc = 0x1125a46; continue;
 }
 0x1125a46 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125a15; continue; }
  pc = 0x1125a4e; continue;
 }
 0x1125a4e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x1125a54; continue;
 }
 0x1125a54 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125a68; continue; }
  pc = 0x1125a62; continue;
 }
 0x1125a62 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125a68; continue;
 }
 0x1125a68 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125aac; continue;
 }
 0x1125a7b => {
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
  pc = 0x1125aac; continue;
 }
 0x1125aac => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125a7b; continue; }
  pc = 0x1125ab4; continue;
 }
 0x1125ab4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125a54; continue; }
  pc = 0x1125ab8; continue;
 }
 0x1125ab8 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125acc; continue; }
  pc = 0x1125ac6; continue;
 }
 0x1125ac6 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125acc; continue;
 }
 0x1125acc => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125b10; continue;
 }
 0x1125adf => {
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
  pc = 0x1125b10; continue;
 }
 0x1125b10 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125adf; continue; }
  pc = 0x1125b18; continue;
 }
 0x1125b18 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125b2d; continue; }
  pc = 0x1125b27; continue;
 }
 0x1125b27 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125b2d; continue;
 }
 0x1125b2d => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125b71; continue;
 }
 0x1125b40 => {
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
  pc = 0x1125b71; continue;
 }
 0x1125b71 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125b40; continue; }
  pc = 0x1125b79; continue;
 }
 0x1125b79 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x1251b60u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x1251bbau32)); }
  if cpu.zf { pc = 0x1125bd0; continue; }
  pc = 0x1125b92; continue;
 }
 0x1125b92 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x12510e4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125bc7; continue; }
  pc = 0x1125bb6; continue;
 }
 0x1125bb6 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125bc7; continue;
 }
 0x1125bc7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x12510e4u32, v); }
  pc = 0x1125bd0; continue;
 }
 0x1125bd0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125beb; continue; }
  pc = 0x1125be5; continue;
 }
 0x1125be5 => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125beb; continue;
 }
 0x1125beb => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125c2f; continue;
 }
 0x1125bfe => {
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
  pc = 0x1125c2f; continue;
 }
 0x1125c2f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125bfe; continue; }
  pc = 0x1125c37; continue;
 }
 0x1125c37 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x46u32; cpu.ebp = v; }
  pc = 0x1125c3d; continue;
 }
 0x1125c3d => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x1300034u32)); }
  if cpu.cf { pc = 0x1125c51; continue; }
  pc = 0x1125c4b; continue;
 }
 0x1125c4b => {
  { let v: u32 = 0x1251100u32; cpu.edx = v; }
  pc = 0x1125c51; continue;
 }
 0x1125c51 => {
  { let v: u32 = cpu.r32(0x1174198u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1300018u32, v); }
  pc = 0x1125c95; continue;
 }
 0x1125c64 => {
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
  pc = 0x1125c95; continue;
 }
 0x1125c95 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x1300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x1125c64; continue; }
  pc = 0x1125c9d; continue;
 }
 0x1125c9d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1125c3d; continue; }
  pc = 0x1125ca1; continue;
 }
 0x1125ca1 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125ca4); }
 let mut pc: u32 = 0x1125ca4;
 loop { match pc {
 0x1125ca4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125cac); }
 let mut pc: u32 = 0x1125cac;
 loop { match pc {
 0x1125cac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125cb4); }
 let mut pc: u32 = 0x1125cb4;
 loop { match pc {
 0x1125cb4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x1125cbd; continue;
 }
 0x1125cbd => {
  pc = 0x1125cbf; continue;
 }
 0x1125cbf => {
  pc = 0x1125cc1; continue;
 }
 0x1125cc1 => {
  {  }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x1125cc3; continue;
 }
 0x1125cc3 => {
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1125ccc; continue;
 }
 0x1125ccc => {
  pc = 0x1125cce; continue;
 }
 0x1125cce => {
  pc = 0x1125cd0; continue;
 }
 0x1125cd0 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1125cde; continue;
 }
 0x1125cde => {
  pc = 0x1125ce0; continue;
 }
 0x1125ce0 => {
  pc = 0x1125ce2; continue;
 }
 0x1125ce2 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x1125cf0; continue;
 }
 0x1125cf0 => {
  pc = 0x1125cf2; continue;
 }
 0x1125cf2 => {
  pc = 0x1125cf4; continue;
 }
 0x1125cf4 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x1125cc3; continue; }
  pc = 0x1125cfc; continue;
 }
 0x1125cfc => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125d26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125d26); }
 let mut pc: u32 = 0x1125d26;
 loop { match pc {
 0x1125d26 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1125d34; continue; }
  pc = 0x1125d2f; continue;
 }
 0x1125d2f => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x1125d2f; continue; }
  pc = 0x1125d34; continue;
 }
 0x1125d34 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x1125d34; continue; }
  pc = 0x1125d39; continue;
 }
 0x1125d39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125d3a); }
 let mut pc: u32 = 0x1125d3a;
 loop { match pc {
 0x1125d3a => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1251bc0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01125d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1125d47); }
 let mut pc: u32 = 0x1125d47;
 loop { match pc {
 0x1125d47 => {
  { let v: u16 = cpu.r16(0x1251bc0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x1251bbau32), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x1125d83; continue; }
  pc = 0x1125d57; continue;
 }
 0x1125d57 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1251bbau32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x1251bbcu32), 0xffffu16); }
  if cpu.zf { pc = 0x1125d83; continue; }
  pc = 0x1125d72; continue;
 }
 0x1125d72 => {
  { let v: u16 = cpu.r16(0x1251bbau32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x1125d83; continue;
 }
 0x1125d83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01130000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1130000); }
 let mut pc: u32 = 0x1130000;
 loop { match pc {
 0x1130000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x1130003; continue;
 }
 0x1130003 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x1130011; continue; }
  pc = 0x113000e; continue;
 }
 0x113000e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x1130011; continue;
 }
 0x1130011 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x1130003; continue; }
  pc = 0x1130020; continue;
 }
 0x1130020 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01130022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1130022); }
 let mut pc: u32 = 0x1130022;
 loop { match pc {
 0x1130022 => {
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
pub fn f_01130038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x1130038); }
 let mut pc: u32 = 0x1130038;
 loop { match pc {
 0x1130038 => {
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
  { let v: u32 = 0x0u32; cpu.w32(0x125109cu32, v); }
  pc = 0x1130064; continue;
 }
 0x1130064 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x125109cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x1130080; continue; }
  pc = 0x1130071; continue;
 }
 0x1130071 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x125109cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x125109cu32, v); }
  pc = 0x1130080; continue;
 }
 0x1130080 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x1130085; continue; }
  pc = 0x1130084; continue;
 }
 0x1130084 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x1130085; continue;
 }
 0x1130085 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x113008a; continue; }
  pc = 0x1130089; continue;
 }
 0x1130089 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x113008a; continue;
 }
 0x113008a => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x113008f; continue; }
  pc = 0x113008e; continue;
 }
 0x113008e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x113008f; continue;
 }
 0x113008f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x1130094; continue; }
  pc = 0x1130093; continue;
 }
 0x1130093 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x1130094; continue;
 }
 0x1130094 => {
  { cpu.and8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x1130064; continue; }
  pc = 0x1130098; continue;
 }
 0x1130098 => {
  {  }
  { let v: u32 = cpu.esi; cpu.w32(0x12510c0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x12510bcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x12510c4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x12510a0u32, v); }
  { let v: u8 = cpu.r8(0x125109cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x125109cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01140000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1140000); } r_01140000(cpu, 0x1140000); }
pub fn f_01140002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1140002); } r_01140000(cpu, 0x1140002); }
pub fn f_01140008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1140008); } r_01140000(cpu, 0x1140008); }
fn r_01140000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1140000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1140002; continue;
 }
 0x1140002 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1140008; continue;
 }
 0x1140008 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.esi; cpu.set_segment(3, v as u16); }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if !cpu.zf { pc = 0x1140014; continue; }
  pc = 0x1140011; continue;
 }
 0x1140011 => {
  cpu.flat_ds(0x1140011); cpu.push(0x1140016); cpu.call(0x970051); if cpu.returned_to != 0x1140016 { return; }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esi); cpu.w32(0x3u32.wrapping_add(cpu.ebp), v); }
  pc = 0x1140019; continue;
 }
 0x1140014 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x114001c; continue; }
  pc = 0x1140019; continue;
 }
 0x1140019 => {
  cpu.flat_ds(0x1140019); cpu.push(0x114001e); cpu.call(0x970065); if cpu.returned_to != 0x114001e { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x3u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x1140021; continue;
 }
 0x114001c => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x1140024; continue; }
  pc = 0x1140021; continue;
 }
 0x1140021 => {
  cpu.flat_ds(0x1140021); cpu.push(0x1140026); cpu.call(0x970073); if cpu.returned_to != 0x1140026 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  pc = 0x1140029; continue;
 }
 0x1140024 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x114002c; continue; }
  pc = 0x1140029; continue;
 }
 0x1140029 => {
  cpu.flat_ds(0x1140029); cpu.push(0x114002e); cpu.call(0x970079); if cpu.returned_to != 0x114002e { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x75u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1140039; continue;
 }
 0x114002c => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x1140034; continue; }
  pc = 0x1140031; continue;
 }
 0x1140031 => {
  cpu.flat_ds(0x1140031); cpu.push(0x1140036); cpu.call(0x97008d); if cpu.returned_to != 0x1140036 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x55e80375u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x1140041; continue;
 }
 0x1140034 => {
  { cpu.sub32(cpu.edi, 0x5u32); }
  if !cpu.zf { pc = 0x114003c; continue; }
  pc = 0x1140039; continue;
 }
 0x1140039 => {
  cpu.flat_ds(0x1140039); cpu.push(0x114003e); cpu.call(0x970093); if cpu.returned_to != 0x114003e { return; }
  { cpu.push(cpu.segments[0] as u32); }
  if !cpu.zf { pc = 0x1140044; continue; }
  pc = 0x1140041; continue;
 }
 0x114003c => {
  { cpu.sub32(cpu.edi, 0x6u32); }
  if !cpu.zf { pc = 0x1140044; continue; }
  pc = 0x1140041; continue;
 }
 0x1140041 => {
  cpu.flat_ds(0x1140041); cpu.push(0x1140046); cpu.call(0x9700a3); if cpu.returned_to != 0x1140046 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if !cpu.zf { pc = 0x114004c; continue; }
  pc = 0x1140049; continue;
 }
 0x1140044 => {
  { cpu.sub32(cpu.edi, 0x7u32); }
  if !cpu.zf { pc = 0x114004c; continue; }
  pc = 0x1140049; continue;
 }
 0x1140049 => {
  cpu.flat_ds(0x1140049); cpu.push(0x114004e); cpu.call(0x83300b1); if cpu.returned_to != 0x114004e { return; }
  cpu.flat_ds(0x114004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x114004c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x114004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0114004f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x114004f); }
 let mut pc: u32 = 0x114004f;
 loop { match pc {
 0x114004f => {
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
pub fn f_01150000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1150000); } r_01150000(cpu, 0x1150000); }
pub fn f_01150002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1150002); } r_01150000(cpu, 0x1150002); }
fn r_01150000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1150000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x1150002; continue;
 }
 0x1150002 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x1150042; continue; }
  pc = 0x1150009; continue;
 }
 0x1150009 => {
  { let v: u32 = 0x66670007u32; cpu.eax = v; }
  { let v: u32 = 0xffffe000u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 16u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x1150042; continue; }
  pc = 0x115001f; continue;
 }
 0x115001f => {
  {  }
  { let v = cpu.ebx; cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x8dfb2000u32; cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x500e0fffu32); cpu.eax = v; }
  { let v = (cpu.edi as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x1150036 => {
  {  }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.ebp as u16); cpu.esp = (cpu.esp & 0xffff0000) | v as u32; }
  {  }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x1150042 => {
  cpu.unimplemented(0x1150042, "Int3");
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x1150036; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_01000000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1000000); } r_01000000(cpu, 0x1000000); }
pub fn f_01000001(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x1000001); } r_01000000(cpu, 0x1000001); }
fn r_01000000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x1000000 => {
  cpu.unimplemented(0x1000000, "Int3");
  pc = 0x1000001; continue;
 }
 0x1000001 => {
  cpu.poll(); pc = 0x1000000; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0101840f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x101840f); }
 let mut pc: u32 = 0x101840f;
 loop { match pc {
 0x10183ef => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x7cu8); }
  if cpu.zf { pc = 0x1018401; continue; }
  pc = 0x10183f8; continue;
 }
 0x10183f8 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x7cu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10183f8; continue; }
  pc = 0x1018401; continue;
 }
 0x1018401 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x101840c; continue; }
  pc = 0x101840a; continue;
 }
 0x101840a => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x101840c; continue;
 }
 0x101840c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x101840f; continue;
 }
 0x101840f => {
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10183ef; continue; }
  pc = 0x1018413; continue;
 }
 0x1018413 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x17u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x15u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x19u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
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
pub fn f_010c3bec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c3bec); }
 let mut pc: u32 = 0x10c3bec;
 loop { match pc {
 0x10c3bec => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x11742edu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c3c3d; continue; }
  pc = 0x10c3c1a; continue;
 }
 0x10c3c1a => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x116b408u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x10c3c3d; continue; }
  pc = 0x10c3c36; continue;
 }
 0x10c3c36 => {
  { let v: u8 = cpu.r8(0x117bb1cu32) | 0x1u8; cpu.w8(0x117bb1cu32, v); }
  pc = 0x10c3c3d; continue;
 }
 0x10c3c3d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c41a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c41a4); }
 let mut pc: u32 = 0x10c41a4;
 loop { match pc {
 0x10c41a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c41a4; continue; }
  pc = 0x10c41b4; continue;
 }
 0x10c41b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c41b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c41b5); }
 let mut pc: u32 = 0x10c41b5;
 loop { match pc {
 0x10c41b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c41b5; continue; }
  pc = 0x10c41c0; continue;
 }
 0x10c41c0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c41c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c41c1); }
 let mut pc: u32 = 0x10c41c1;
 loop { match pc {
 0x10c41c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c41c1; continue; }
  pc = 0x10c41df; continue;
 }
 0x10c41df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c41e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c41e0); }
 let mut pc: u32 = 0x10c41e0;
 loop { match pc {
 0x10c41e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c41e0; continue; }
  pc = 0x10c41f3; continue;
 }
 0x10c41f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c41f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c41f4); }
 let mut pc: u32 = 0x10c41f4;
 loop { match pc {
 0x10c41f4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c41f4; continue; }
  pc = 0x10c4212; continue;
 }
 0x10c4212 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4213(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4213); }
 let mut pc: u32 = 0x10c4213;
 loop { match pc {
 0x10c4213 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4213; continue; }
  pc = 0x10c4226; continue;
 }
 0x10c4226 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4227(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4227); }
 let mut pc: u32 = 0x10c4227;
 loop { match pc {
 0x10c4227 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c4227; continue; }
  pc = 0x10c4243; continue;
 }
 0x10c4243 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4244(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4244); }
 let mut pc: u32 = 0x10c4244;
 loop { match pc {
 0x10c4244 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4244; continue; }
  pc = 0x10c4255; continue;
 }
 0x10c4255 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4256(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4256); }
 let mut pc: u32 = 0x10c4256;
 loop { match pc {
 0x10c4256 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c4256; continue; }
  pc = 0x10c4272; continue;
 }
 0x10c4272 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4273(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4273); }
 let mut pc: u32 = 0x10c4273;
 loop { match pc {
 0x10c4273 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4273; continue; }
  pc = 0x10c4284; continue;
 }
 0x10c4284 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4285); }
 let mut pc: u32 = 0x10c4285;
 loop { match pc {
 0x10c4285 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4285; continue; }
  pc = 0x10c4298; continue;
 }
 0x10c4298 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4299(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4299); }
 let mut pc: u32 = 0x10c4299;
 loop { match pc {
 0x10c4299 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4299; continue; }
  pc = 0x10c42a7; continue;
 }
 0x10c42a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c42a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c42a8); }
 let mut pc: u32 = 0x10c42a8;
 loop { match pc {
 0x10c42a8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c42a8; continue; }
  pc = 0x10c42bd; continue;
 }
 0x10c42bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c42be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c42be); }
 let mut pc: u32 = 0x10c42be;
 loop { match pc {
 0x10c42be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c42be; continue; }
  pc = 0x10c42ce; continue;
 }
 0x10c42ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c42cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c42cf); }
 let mut pc: u32 = 0x10c42cf;
 loop { match pc {
 0x10c42cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c42cf; continue; }
  pc = 0x10c42ef; continue;
 }
 0x10c42ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c42f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c42f0); }
 let mut pc: u32 = 0x10c42f0;
 loop { match pc {
 0x10c42f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c42f0; continue; }
  pc = 0x10c4305; continue;
 }
 0x10c4305 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4306(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4306); }
 let mut pc: u32 = 0x10c4306;
 loop { match pc {
 0x10c4306 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4306; continue; }
  pc = 0x10c4326; continue;
 }
 0x10c4326 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4327(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4327); }
 let mut pc: u32 = 0x10c4327;
 loop { match pc {
 0x10c4327 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4327; continue; }
  pc = 0x10c433c; continue;
 }
 0x10c433c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c433d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c433d); }
 let mut pc: u32 = 0x10c433d;
 loop { match pc {
 0x10c433d => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c433d; continue; }
  pc = 0x10c4361; continue;
 }
 0x10c4361 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4362(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4362); }
 let mut pc: u32 = 0x10c4362;
 loop { match pc {
 0x10c4362 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4362; continue; }
  pc = 0x10c437b; continue;
 }
 0x10c437b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c437c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c437c); }
 let mut pc: u32 = 0x10c437c;
 loop { match pc {
 0x10c437c => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c437c; continue; }
  pc = 0x10c43a0; continue;
 }
 0x10c43a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c43a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c43a1); }
 let mut pc: u32 = 0x10c43a1;
 loop { match pc {
 0x10c43a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c43a1; continue; }
  pc = 0x10c43ba; continue;
 }
 0x10c43ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c43bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c43bb); }
 let mut pc: u32 = 0x10c43bb;
 loop { match pc {
 0x10c43bb => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c43bb; continue; }
  pc = 0x10c43dd; continue;
 }
 0x10c43dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c43de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c43de); }
 let mut pc: u32 = 0x10c43de;
 loop { match pc {
 0x10c43de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c43de; continue; }
  pc = 0x10c43f5; continue;
 }
 0x10c43f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c43f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c43f6); }
 let mut pc: u32 = 0x10c43f6;
 loop { match pc {
 0x10c43f6 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c43f6; continue; }
  pc = 0x10c4418; continue;
 }
 0x10c4418 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4419(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4419); }
 let mut pc: u32 = 0x10c4419;
 loop { match pc {
 0x10c4419 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4419; continue; }
  pc = 0x10c4430; continue;
 }
 0x10c4430 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4431(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4431); }
 let mut pc: u32 = 0x10c4431;
 loop { match pc {
 0x10c4431 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c4431; continue; }
  pc = 0x10c4457; continue;
 }
 0x10c4457 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4458); }
 let mut pc: u32 = 0x10c4458;
 loop { match pc {
 0x10c4458 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c4458; continue; }
  pc = 0x10c4473; continue;
 }
 0x10c4473 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4474); }
 let mut pc: u32 = 0x10c4474;
 loop { match pc {
 0x10c4474 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c4474; continue; }
  pc = 0x10c449a; continue;
 }
 0x10c449a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c449b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c449b); }
 let mut pc: u32 = 0x10c449b;
 loop { match pc {
 0x10c449b => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10c449b; continue; }
  pc = 0x10c44b6; continue;
 }
 0x10c44b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c44b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c44b7); }
 let mut pc: u32 = 0x10c44b7;
 loop { match pc {
 0x10c44b7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c44d6; continue; }
  pc = 0x10c44c3; continue;
 }
 0x10c44c3 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c44dd; continue; }
  pc = 0x10c44ca; continue;
 }
 0x10c44ca => {
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c44b7; continue; }
  pc = 0x10c44d5; continue;
 }
 0x10c44d5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c44d6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c44ca; continue;
 }
 0x10c44dd => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c44ca; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c44e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c44e4); }
 let mut pc: u32 = 0x10c44e4;
 loop { match pc {
 0x10c44e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c44fe; continue; }
  pc = 0x10c44f0; continue;
 }
 0x10c44f0 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4505; continue; }
  pc = 0x10c44f7; continue;
 }
 0x10c44f7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c44e4; continue; }
  pc = 0x10c44fd; continue;
 }
 0x10c44fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c44fe => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c44f7; continue;
 }
 0x10c4505 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c44f7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c450c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c450c); }
 let mut pc: u32 = 0x10c450c;
 loop { match pc {
 0x10c450c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4549; continue; }
  pc = 0x10c451b; continue;
 }
 0x10c451b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4550; continue; }
  pc = 0x10c4522; continue;
 }
 0x10c4522 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4557; continue; }
  pc = 0x10c452a; continue;
 }
 0x10c452a => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c455e; continue; }
  pc = 0x10c4532; continue;
 }
 0x10c4532 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c450c; continue; }
  pc = 0x10c4548; continue;
 }
 0x10c4548 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4549 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4522; continue;
 }
 0x10c4550 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4522; continue;
 }
 0x10c4557 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4532; continue;
 }
 0x10c455e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4532; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4565(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4565); }
 let mut pc: u32 = 0x10c4565;
 loop { match pc {
 0x10c4565 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4597; continue; }
  pc = 0x10c4574; continue;
 }
 0x10c4574 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c459e; continue; }
  pc = 0x10c457b; continue;
 }
 0x10c457b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c45a5; continue; }
  pc = 0x10c4583; continue;
 }
 0x10c4583 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c45ac; continue; }
  pc = 0x10c458b; continue;
 }
 0x10c458b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4565; continue; }
  pc = 0x10c4596; continue;
 }
 0x10c4596 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4597 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c457b; continue;
 }
 0x10c459e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c457b; continue;
 }
 0x10c45a5 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c458b; continue;
 }
 0x10c45ac => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c458b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c45b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c45b3); }
 let mut pc: u32 = 0x10c45b3;
 loop { match pc {
 0x10c45b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c45f0; continue; }
  pc = 0x10c45c2; continue;
 }
 0x10c45c2 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c45f7; continue; }
  pc = 0x10c45c9; continue;
 }
 0x10c45c9 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c45fe; continue; }
  pc = 0x10c45d1; continue;
 }
 0x10c45d1 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4605; continue; }
  pc = 0x10c45d9; continue;
 }
 0x10c45d9 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c45b3; continue; }
  pc = 0x10c45ef; continue;
 }
 0x10c45ef => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c45f0 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c45c9; continue;
 }
 0x10c45f7 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c45c9; continue;
 }
 0x10c45fe => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c45d9; continue;
 }
 0x10c4605 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c45d9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c460c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c460c); }
 let mut pc: u32 = 0x10c460c;
 loop { match pc {
 0x10c460c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c463e; continue; }
  pc = 0x10c461b; continue;
 }
 0x10c461b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4645; continue; }
  pc = 0x10c4622; continue;
 }
 0x10c4622 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c464c; continue; }
  pc = 0x10c462a; continue;
 }
 0x10c462a => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4653; continue; }
  pc = 0x10c4632; continue;
 }
 0x10c4632 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c460c; continue; }
  pc = 0x10c463d; continue;
 }
 0x10c463d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c463e => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4622; continue;
 }
 0x10c4645 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4622; continue;
 }
 0x10c464c => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4632; continue;
 }
 0x10c4653 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4632; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c465a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c465a); }
 let mut pc: u32 = 0x10c465a;
 loop { match pc {
 0x10c465a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4695; continue; }
  pc = 0x10c4669; continue;
 }
 0x10c4669 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c469c; continue; }
  pc = 0x10c4670; continue;
 }
 0x10c4670 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c46a3; continue; }
  pc = 0x10c4678; continue;
 }
 0x10c4678 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c46aa; continue; }
  pc = 0x10c4680; continue;
 }
 0x10c4680 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c465a; continue; }
  pc = 0x10c4694; continue;
 }
 0x10c4694 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4695 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4670; continue;
 }
 0x10c469c => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4670; continue;
 }
 0x10c46a3 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4680; continue;
 }
 0x10c46aa => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4680; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c46b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c46b1); }
 let mut pc: u32 = 0x10c46b1;
 loop { match pc {
 0x10c46b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c46e1; continue; }
  pc = 0x10c46c0; continue;
 }
 0x10c46c0 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c46e8; continue; }
  pc = 0x10c46c7; continue;
 }
 0x10c46c7 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c46ef; continue; }
  pc = 0x10c46cf; continue;
 }
 0x10c46cf => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c46f6; continue; }
  pc = 0x10c46d7; continue;
 }
 0x10c46d7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c46b1; continue; }
  pc = 0x10c46e0; continue;
 }
 0x10c46e0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c46e1 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c46c7; continue;
 }
 0x10c46e8 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c46c7; continue;
 }
 0x10c46ef => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c46d7; continue;
 }
 0x10c46f6 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c46d7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c46fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c46fd); }
 let mut pc: u32 = 0x10c46fd;
 loop { match pc {
 0x10c46fd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4738; continue; }
  pc = 0x10c470c; continue;
 }
 0x10c470c => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c473f; continue; }
  pc = 0x10c4713; continue;
 }
 0x10c4713 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4746; continue; }
  pc = 0x10c471b; continue;
 }
 0x10c471b => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c474d; continue; }
  pc = 0x10c4723; continue;
 }
 0x10c4723 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c46fd; continue; }
  pc = 0x10c4737; continue;
 }
 0x10c4737 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4738 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4713; continue;
 }
 0x10c473f => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4713; continue;
 }
 0x10c4746 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4723; continue;
 }
 0x10c474d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4723; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4754(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4754); }
 let mut pc: u32 = 0x10c4754;
 loop { match pc {
 0x10c4754 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4784; continue; }
  pc = 0x10c4763; continue;
 }
 0x10c4763 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c478b; continue; }
  pc = 0x10c476a; continue;
 }
 0x10c476a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4792; continue; }
  pc = 0x10c4772; continue;
 }
 0x10c4772 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4799; continue; }
  pc = 0x10c477a; continue;
 }
 0x10c477a => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4754; continue; }
  pc = 0x10c4783; continue;
 }
 0x10c4783 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4784 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c476a; continue;
 }
 0x10c478b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c476a; continue;
 }
 0x10c4792 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c477a; continue;
 }
 0x10c4799 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c477a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c47a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c47a0); }
 let mut pc: u32 = 0x10c47a0;
 loop { match pc {
 0x10c47a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c47c2; continue; }
  pc = 0x10c47ac; continue;
 }
 0x10c47ac => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c47c9; continue; }
  pc = 0x10c47b3; continue;
 }
 0x10c47b3 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c47a0; continue; }
  pc = 0x10c47c1; continue;
 }
 0x10c47c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c47c2 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c47b3; continue;
 }
 0x10c47c9 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c47b3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c47d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c47d0); }
 let mut pc: u32 = 0x10c47d0;
 loop { match pc {
 0x10c47d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c47ed; continue; }
  pc = 0x10c47dc; continue;
 }
 0x10c47dc => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c47f4; continue; }
  pc = 0x10c47e3; continue;
 }
 0x10c47e3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c47d0; continue; }
  pc = 0x10c47ec; continue;
 }
 0x10c47ec => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c47ed => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c47e3; continue;
 }
 0x10c47f4 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c47e3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c47fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c47fb); }
 let mut pc: u32 = 0x10c47fb;
 loop { match pc {
 0x10c47fb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c481f; continue; }
  pc = 0x10c4807; continue;
 }
 0x10c4807 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4826; continue; }
  pc = 0x10c480e; continue;
 }
 0x10c480e => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c47fb; continue; }
  pc = 0x10c481e; continue;
 }
 0x10c481e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c481f => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c480e; continue;
 }
 0x10c4826 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c480e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c482d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c482d); }
 let mut pc: u32 = 0x10c482d;
 loop { match pc {
 0x10c482d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c484c; continue; }
  pc = 0x10c4839; continue;
 }
 0x10c4839 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4853; continue; }
  pc = 0x10c4840; continue;
 }
 0x10c4840 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c482d; continue; }
  pc = 0x10c484b; continue;
 }
 0x10c484b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c484c => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4840; continue;
 }
 0x10c4853 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4840; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c485a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c485a); }
 let mut pc: u32 = 0x10c485a;
 loop { match pc {
 0x10c485a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4899; continue; }
  pc = 0x10c4869; continue;
 }
 0x10c4869 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c48a0; continue; }
  pc = 0x10c4870; continue;
 }
 0x10c4870 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c48a7; continue; }
  pc = 0x10c4878; continue;
 }
 0x10c4878 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c48ae; continue; }
  pc = 0x10c4880; continue;
 }
 0x10c4880 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c485a; continue; }
  pc = 0x10c4898; continue;
 }
 0x10c4898 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4899 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4870; continue;
 }
 0x10c48a0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4870; continue;
 }
 0x10c48a7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4880; continue;
 }
 0x10c48ae => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4880; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c48b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c48b5); }
 let mut pc: u32 = 0x10c48b5;
 loop { match pc {
 0x10c48b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c48e9; continue; }
  pc = 0x10c48c4; continue;
 }
 0x10c48c4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c48f0; continue; }
  pc = 0x10c48cb; continue;
 }
 0x10c48cb => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c48f7; continue; }
  pc = 0x10c48d3; continue;
 }
 0x10c48d3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c48fe; continue; }
  pc = 0x10c48db; continue;
 }
 0x10c48db => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c48b5; continue; }
  pc = 0x10c48e8; continue;
 }
 0x10c48e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c48e9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c48cb; continue;
 }
 0x10c48f0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c48cb; continue;
 }
 0x10c48f7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c48db; continue;
 }
 0x10c48fe => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c48db; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4905(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4905); }
 let mut pc: u32 = 0x10c4905;
 loop { match pc {
 0x10c4905 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4944; continue; }
  pc = 0x10c4914; continue;
 }
 0x10c4914 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c494b; continue; }
  pc = 0x10c491b; continue;
 }
 0x10c491b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4952; continue; }
  pc = 0x10c4923; continue;
 }
 0x10c4923 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4959; continue; }
  pc = 0x10c492b; continue;
 }
 0x10c492b => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4905; continue; }
  pc = 0x10c4943; continue;
 }
 0x10c4943 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4944 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c491b; continue;
 }
 0x10c494b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c491b; continue;
 }
 0x10c4952 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c492b; continue;
 }
 0x10c4959 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c492b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4960); }
 let mut pc: u32 = 0x10c4960;
 loop { match pc {
 0x10c4960 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4994; continue; }
  pc = 0x10c496f; continue;
 }
 0x10c496f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c499b; continue; }
  pc = 0x10c4976; continue;
 }
 0x10c4976 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c49a2; continue; }
  pc = 0x10c497e; continue;
 }
 0x10c497e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c49a9; continue; }
  pc = 0x10c4986; continue;
 }
 0x10c4986 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4960; continue; }
  pc = 0x10c4993; continue;
 }
 0x10c4993 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4994 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4976; continue;
 }
 0x10c499b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4976; continue;
 }
 0x10c49a2 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4986; continue;
 }
 0x10c49a9 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4986; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c49b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c49b0); }
 let mut pc: u32 = 0x10c49b0;
 loop { match pc {
 0x10c49b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c49f3; continue; }
  pc = 0x10c49bf; continue;
 }
 0x10c49bf => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c49fa; continue; }
  pc = 0x10c49c6; continue;
 }
 0x10c49c6 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4a01; continue; }
  pc = 0x10c49ce; continue;
 }
 0x10c49ce => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4a08; continue; }
  pc = 0x10c49d6; continue;
 }
 0x10c49d6 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c49b0; continue; }
  pc = 0x10c49f2; continue;
 }
 0x10c49f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c49f3 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c49c6; continue;
 }
 0x10c49fa => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c49c6; continue;
 }
 0x10c4a01 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c49d6; continue;
 }
 0x10c4a08 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c49d6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4a0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4a0f); }
 let mut pc: u32 = 0x10c4a0f;
 loop { match pc {
 0x10c4a0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4a47; continue; }
  pc = 0x10c4a1e; continue;
 }
 0x10c4a1e => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4a4e; continue; }
  pc = 0x10c4a25; continue;
 }
 0x10c4a25 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4a55; continue; }
  pc = 0x10c4a2d; continue;
 }
 0x10c4a2d => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4a5c; continue; }
  pc = 0x10c4a35; continue;
 }
 0x10c4a35 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4a0f; continue; }
  pc = 0x10c4a46; continue;
 }
 0x10c4a46 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4a47 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4a25; continue;
 }
 0x10c4a4e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4a25; continue;
 }
 0x10c4a55 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4a35; continue;
 }
 0x10c4a5c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4a35; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4a63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4a63); }
 let mut pc: u32 = 0x10c4a63;
 loop { match pc {
 0x10c4a63 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4aa6; continue; }
  pc = 0x10c4a72; continue;
 }
 0x10c4a72 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4aad; continue; }
  pc = 0x10c4a79; continue;
 }
 0x10c4a79 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4ab4; continue; }
  pc = 0x10c4a81; continue;
 }
 0x10c4a81 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4abb; continue; }
  pc = 0x10c4a89; continue;
 }
 0x10c4a89 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4a63; continue; }
  pc = 0x10c4aa5; continue;
 }
 0x10c4aa5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4aa6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4a79; continue;
 }
 0x10c4aad => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4a79; continue;
 }
 0x10c4ab4 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4a89; continue;
 }
 0x10c4abb => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4a89; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4ac2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4ac2); }
 let mut pc: u32 = 0x10c4ac2;
 loop { match pc {
 0x10c4ac2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4afa; continue; }
  pc = 0x10c4ad1; continue;
 }
 0x10c4ad1 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4b01; continue; }
  pc = 0x10c4ad8; continue;
 }
 0x10c4ad8 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4b08; continue; }
  pc = 0x10c4ae0; continue;
 }
 0x10c4ae0 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4b0f; continue; }
  pc = 0x10c4ae8; continue;
 }
 0x10c4ae8 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4ac2; continue; }
  pc = 0x10c4af9; continue;
 }
 0x10c4af9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4afa => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4ad8; continue;
 }
 0x10c4b01 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4ad8; continue;
 }
 0x10c4b08 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4ae8; continue;
 }
 0x10c4b0f => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4ae8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4b16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4b16); }
 let mut pc: u32 = 0x10c4b16;
 loop { match pc {
 0x10c4b16 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4b57; continue; }
  pc = 0x10c4b25; continue;
 }
 0x10c4b25 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4b5e; continue; }
  pc = 0x10c4b2c; continue;
 }
 0x10c4b2c => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4b65; continue; }
  pc = 0x10c4b34; continue;
 }
 0x10c4b34 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4b6c; continue; }
  pc = 0x10c4b3c; continue;
 }
 0x10c4b3c => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4b16; continue; }
  pc = 0x10c4b56; continue;
 }
 0x10c4b56 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4b57 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4b2c; continue;
 }
 0x10c4b5e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4b2c; continue;
 }
 0x10c4b65 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4b3c; continue;
 }
 0x10c4b6c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4b3c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4b73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4b73); }
 let mut pc: u32 = 0x10c4b73;
 loop { match pc {
 0x10c4b73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4ba9; continue; }
  pc = 0x10c4b82; continue;
 }
 0x10c4b82 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4bb0; continue; }
  pc = 0x10c4b89; continue;
 }
 0x10c4b89 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4bb7; continue; }
  pc = 0x10c4b91; continue;
 }
 0x10c4b91 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4bbe; continue; }
  pc = 0x10c4b99; continue;
 }
 0x10c4b99 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4b73; continue; }
  pc = 0x10c4ba8; continue;
 }
 0x10c4ba8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4ba9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4b89; continue;
 }
 0x10c4bb0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4b89; continue;
 }
 0x10c4bb7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4b99; continue;
 }
 0x10c4bbe => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4b99; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4bc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4bc5); }
 let mut pc: u32 = 0x10c4bc5;
 loop { match pc {
 0x10c4bc5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4c06; continue; }
  pc = 0x10c4bd4; continue;
 }
 0x10c4bd4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4c0d; continue; }
  pc = 0x10c4bdb; continue;
 }
 0x10c4bdb => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4c14; continue; }
  pc = 0x10c4be3; continue;
 }
 0x10c4be3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4c1b; continue; }
  pc = 0x10c4beb; continue;
 }
 0x10c4beb => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4bc5; continue; }
  pc = 0x10c4c05; continue;
 }
 0x10c4c05 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4c06 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4bdb; continue;
 }
 0x10c4c0d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4bdb; continue;
 }
 0x10c4c14 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4beb; continue;
 }
 0x10c4c1b => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4beb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4c22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4c22); }
 let mut pc: u32 = 0x10c4c22;
 loop { match pc {
 0x10c4c22 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4c58; continue; }
  pc = 0x10c4c31; continue;
 }
 0x10c4c31 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4c5f; continue; }
  pc = 0x10c4c38; continue;
 }
 0x10c4c38 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4c66; continue; }
  pc = 0x10c4c40; continue;
 }
 0x10c4c40 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4c6d; continue; }
  pc = 0x10c4c48; continue;
 }
 0x10c4c48 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4c22; continue; }
  pc = 0x10c4c57; continue;
 }
 0x10c4c57 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4c58 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4c38; continue;
 }
 0x10c4c5f => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4c38; continue;
 }
 0x10c4c66 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4c48; continue;
 }
 0x10c4c6d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4c48; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4c74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4c74); }
 let mut pc: u32 = 0x10c4c74;
 loop { match pc {
 0x10c4c74 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4cb9; continue; }
  pc = 0x10c4c83; continue;
 }
 0x10c4c83 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4cc0; continue; }
  pc = 0x10c4c8a; continue;
 }
 0x10c4c8a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4cc7; continue; }
  pc = 0x10c4c92; continue;
 }
 0x10c4c92 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4cce; continue; }
  pc = 0x10c4c9a; continue;
 }
 0x10c4c9a => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4c74; continue; }
  pc = 0x10c4cb8; continue;
 }
 0x10c4cb8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4cb9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4c8a; continue;
 }
 0x10c4cc0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4c8a; continue;
 }
 0x10c4cc7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4c9a; continue;
 }
 0x10c4cce => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4c9a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4cd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4cd5); }
 let mut pc: u32 = 0x10c4cd5;
 loop { match pc {
 0x10c4cd5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4d0f; continue; }
  pc = 0x10c4ce4; continue;
 }
 0x10c4ce4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4d16; continue; }
  pc = 0x10c4ceb; continue;
 }
 0x10c4ceb => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4d1d; continue; }
  pc = 0x10c4cf3; continue;
 }
 0x10c4cf3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4d24; continue; }
  pc = 0x10c4cfb; continue;
 }
 0x10c4cfb => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4cd5; continue; }
  pc = 0x10c4d0e; continue;
 }
 0x10c4d0e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4d0f => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4ceb; continue;
 }
 0x10c4d16 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4ceb; continue;
 }
 0x10c4d1d => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4cfb; continue;
 }
 0x10c4d24 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4cfb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4d2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4d2b); }
 let mut pc: u32 = 0x10c4d2b;
 loop { match pc {
 0x10c4d2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4d70; continue; }
  pc = 0x10c4d3a; continue;
 }
 0x10c4d3a => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4d77; continue; }
  pc = 0x10c4d41; continue;
 }
 0x10c4d41 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4d7e; continue; }
  pc = 0x10c4d49; continue;
 }
 0x10c4d49 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4d85; continue; }
  pc = 0x10c4d51; continue;
 }
 0x10c4d51 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4d2b; continue; }
  pc = 0x10c4d6f; continue;
 }
 0x10c4d6f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4d70 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4d41; continue;
 }
 0x10c4d77 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4d41; continue;
 }
 0x10c4d7e => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4d51; continue;
 }
 0x10c4d85 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4d51; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4d8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4d8c); }
 let mut pc: u32 = 0x10c4d8c;
 loop { match pc {
 0x10c4d8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4dc6; continue; }
  pc = 0x10c4d9b; continue;
 }
 0x10c4d9b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4dcd; continue; }
  pc = 0x10c4da2; continue;
 }
 0x10c4da2 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10c4dd4; continue; }
  pc = 0x10c4daa; continue;
 }
 0x10c4daa => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x10c4ddb; continue; }
  pc = 0x10c4db2; continue;
 }
 0x10c4db2 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10c4d8c; continue; }
  pc = 0x10c4dc5; continue;
 }
 0x10c4dc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c4dc6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4da2; continue;
 }
 0x10c4dcd => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10c4da2; continue;
 }
 0x10c4dd4 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4db2; continue;
 }
 0x10c4ddb => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x10c4db2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4de2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4de2); }
 let mut pc: u32 = 0x10c4de2;
 loop { match pc {
 0x10c4de2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4e02; continue; }
  pc = 0x10c4dea; continue;
 }
 0x10c4dea => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4de2; continue; }
  pc = 0x10c4e02; continue;
 }
 0x10c4e02 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4e03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4e03); }
 let mut pc: u32 = 0x10c4e03;
 loop { match pc {
 0x10c4e03 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4e26; continue; }
  pc = 0x10c4e0b; continue;
 }
 0x10c4e0b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4e03; continue; }
  pc = 0x10c4e26; continue;
 }
 0x10c4e26 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4e27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4e27); }
 let mut pc: u32 = 0x10c4e27;
 loop { match pc {
 0x10c4e27 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4e58; continue; }
  pc = 0x10c4e2f; continue;
 }
 0x10c4e2f => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4e27; continue; }
  pc = 0x10c4e58; continue;
 }
 0x10c4e58 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4e59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4e59); }
 let mut pc: u32 = 0x10c4e59;
 loop { match pc {
 0x10c4e59 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4e8a; continue; }
  pc = 0x10c4e61; continue;
 }
 0x10c4e61 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4e59; continue; }
  pc = 0x10c4e8a; continue;
 }
 0x10c4e8a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4e8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4e8b); }
 let mut pc: u32 = 0x10c4e8b;
 loop { match pc {
 0x10c4e8b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4eae; continue; }
  pc = 0x10c4e93; continue;
 }
 0x10c4e93 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4e8b; continue; }
  pc = 0x10c4eae; continue;
 }
 0x10c4eae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4eaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4eaf); }
 let mut pc: u32 = 0x10c4eaf;
 loop { match pc {
 0x10c4eaf => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4ed5; continue; }
  pc = 0x10c4eb7; continue;
 }
 0x10c4eb7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4eaf; continue; }
  pc = 0x10c4ed5; continue;
 }
 0x10c4ed5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4ed6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4ed6); }
 let mut pc: u32 = 0x10c4ed6;
 loop { match pc {
 0x10c4ed6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4f09; continue; }
  pc = 0x10c4ede; continue;
 }
 0x10c4ede => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4ed6; continue; }
  pc = 0x10c4f09; continue;
 }
 0x10c4f09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4f0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4f0a); }
 let mut pc: u32 = 0x10c4f0a;
 loop { match pc {
 0x10c4f0a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4f3d; continue; }
  pc = 0x10c4f12; continue;
 }
 0x10c4f12 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4f0a; continue; }
  pc = 0x10c4f3d; continue;
 }
 0x10c4f3d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4f3e); }
 let mut pc: u32 = 0x10c4f3e;
 loop { match pc {
 0x10c4f3e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4f59; continue; }
  pc = 0x10c4f46; continue;
 }
 0x10c4f46 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4f3e; continue; }
  pc = 0x10c4f59; continue;
 }
 0x10c4f59 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4f5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4f5a); }
 let mut pc: u32 = 0x10c4f5a;
 loop { match pc {
 0x10c4f5a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4f78; continue; }
  pc = 0x10c4f62; continue;
 }
 0x10c4f62 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4f5a; continue; }
  pc = 0x10c4f78; continue;
 }
 0x10c4f78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4f79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4f79); }
 let mut pc: u32 = 0x10c4f79;
 loop { match pc {
 0x10c4f79 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4f9f; continue; }
  pc = 0x10c4f81; continue;
 }
 0x10c4f81 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4f79; continue; }
  pc = 0x10c4f9f; continue;
 }
 0x10c4f9f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4fa0); }
 let mut pc: u32 = 0x10c4fa0;
 loop { match pc {
 0x10c4fa0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4fc6; continue; }
  pc = 0x10c4fa8; continue;
 }
 0x10c4fa8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4fa0; continue; }
  pc = 0x10c4fc6; continue;
 }
 0x10c4fc6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4fc7); }
 let mut pc: u32 = 0x10c4fc7;
 loop { match pc {
 0x10c4fc7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c4fe5; continue; }
  pc = 0x10c4fcf; continue;
 }
 0x10c4fcf => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4fc7; continue; }
  pc = 0x10c4fe5; continue;
 }
 0x10c4fe5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c4fe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c4fe6); }
 let mut pc: u32 = 0x10c4fe6;
 loop { match pc {
 0x10c4fe6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5007; continue; }
  pc = 0x10c4fee; continue;
 }
 0x10c4fee => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c4fe6; continue; }
  pc = 0x10c5007; continue;
 }
 0x10c5007 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5008(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5008); }
 let mut pc: u32 = 0x10c5008;
 loop { match pc {
 0x10c5008 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5030; continue; }
  pc = 0x10c5010; continue;
 }
 0x10c5010 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5008; continue; }
  pc = 0x10c5030; continue;
 }
 0x10c5030 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5031(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5031); }
 let mut pc: u32 = 0x10c5031;
 loop { match pc {
 0x10c5031 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5059; continue; }
  pc = 0x10c5039; continue;
 }
 0x10c5039 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5031; continue; }
  pc = 0x10c5059; continue;
 }
 0x10c5059 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c505a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c505a); }
 let mut pc: u32 = 0x10c505a;
 loop { match pc {
 0x10c505a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5085; continue; }
  pc = 0x10c5062; continue;
 }
 0x10c5062 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c505a; continue; }
  pc = 0x10c5085; continue;
 }
 0x10c5085 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5086(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5086); }
 let mut pc: u32 = 0x10c5086;
 loop { match pc {
 0x10c5086 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c50b4; continue; }
  pc = 0x10c508e; continue;
 }
 0x10c508e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5086; continue; }
  pc = 0x10c50b4; continue;
 }
 0x10c50b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c50b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c50b5); }
 let mut pc: u32 = 0x10c50b5;
 loop { match pc {
 0x10c50b5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c50f4; continue; }
  pc = 0x10c50bd; continue;
 }
 0x10c50bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c50b5; continue; }
  pc = 0x10c50f4; continue;
 }
 0x10c50f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c50f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c50f5); }
 let mut pc: u32 = 0x10c50f5;
 loop { match pc {
 0x10c50f5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5134; continue; }
  pc = 0x10c50fd; continue;
 }
 0x10c50fd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c50f5; continue; }
  pc = 0x10c5134; continue;
 }
 0x10c5134 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5135(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5135); }
 let mut pc: u32 = 0x10c5135;
 loop { match pc {
 0x10c5135 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5166; continue; }
  pc = 0x10c513d; continue;
 }
 0x10c513d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5135; continue; }
  pc = 0x10c5166; continue;
 }
 0x10c5166 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5167(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5167); }
 let mut pc: u32 = 0x10c5167;
 loop { match pc {
 0x10c5167 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c519b; continue; }
  pc = 0x10c516f; continue;
 }
 0x10c516f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5167; continue; }
  pc = 0x10c519b; continue;
 }
 0x10c519b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c519c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c519c); }
 let mut pc: u32 = 0x10c519c;
 loop { match pc {
 0x10c519c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c51dd; continue; }
  pc = 0x10c51a4; continue;
 }
 0x10c51a4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c519c; continue; }
  pc = 0x10c51dd; continue;
 }
 0x10c51dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c51de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c51de); }
 let mut pc: u32 = 0x10c51de;
 loop { match pc {
 0x10c51de => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c521f; continue; }
  pc = 0x10c51e6; continue;
 }
 0x10c51e6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c51de; continue; }
  pc = 0x10c521f; continue;
 }
 0x10c521f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5220); }
 let mut pc: u32 = 0x10c5220;
 loop { match pc {
 0x10c5220 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5246; continue; }
  pc = 0x10c5228; continue;
 }
 0x10c5228 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5220; continue; }
  pc = 0x10c5246; continue;
 }
 0x10c5246 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5247(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5247); }
 let mut pc: u32 = 0x10c5247;
 loop { match pc {
 0x10c5247 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5270; continue; }
  pc = 0x10c524f; continue;
 }
 0x10c524f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5247; continue; }
  pc = 0x10c5270; continue;
 }
 0x10c5270 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5271(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5271); }
 let mut pc: u32 = 0x10c5271;
 loop { match pc {
 0x10c5271 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c52a5; continue; }
  pc = 0x10c5279; continue;
 }
 0x10c5279 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5271; continue; }
  pc = 0x10c52a5; continue;
 }
 0x10c52a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c52a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c52a6); }
 let mut pc: u32 = 0x10c52a6;
 loop { match pc {
 0x10c52a6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c52da; continue; }
  pc = 0x10c52ae; continue;
 }
 0x10c52ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c52a6; continue; }
  pc = 0x10c52da; continue;
 }
 0x10c52da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c52db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c52db); }
 let mut pc: u32 = 0x10c52db;
 loop { match pc {
 0x10c52db => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5307; continue; }
  pc = 0x10c52e3; continue;
 }
 0x10c52e3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c52db; continue; }
  pc = 0x10c5307; continue;
 }
 0x10c5307 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5308); }
 let mut pc: u32 = 0x10c5308;
 loop { match pc {
 0x10c5308 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5337; continue; }
  pc = 0x10c5310; continue;
 }
 0x10c5310 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5308; continue; }
  pc = 0x10c5337; continue;
 }
 0x10c5337 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5338); }
 let mut pc: u32 = 0x10c5338;
 loop { match pc {
 0x10c5338 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c536e; continue; }
  pc = 0x10c5340; continue;
 }
 0x10c5340 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5338; continue; }
  pc = 0x10c536e; continue;
 }
 0x10c536e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c536f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c536f); }
 let mut pc: u32 = 0x10c536f;
 loop { match pc {
 0x10c536f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c53a5; continue; }
  pc = 0x10c5377; continue;
 }
 0x10c5377 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c536f; continue; }
  pc = 0x10c53a5; continue;
 }
 0x10c53a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c53a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c53a6); }
 let mut pc: u32 = 0x10c53a6;
 loop { match pc {
 0x10c53a6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c53c1; continue; }
  pc = 0x10c53ae; continue;
 }
 0x10c53ae => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c53a6; continue; }
  pc = 0x10c53c1; continue;
 }
 0x10c53c1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c53c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c53c2); }
 let mut pc: u32 = 0x10c53c2;
 loop { match pc {
 0x10c53c2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c53e7; continue; }
  pc = 0x10c53ca; continue;
 }
 0x10c53ca => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c53c2; continue; }
  pc = 0x10c53e7; continue;
 }
 0x10c53e7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c53e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c53e8); }
 let mut pc: u32 = 0x10c53e8;
 loop { match pc {
 0x10c53e8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5411; continue; }
  pc = 0x10c53f0; continue;
 }
 0x10c53f0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c53e8; continue; }
  pc = 0x10c5411; continue;
 }
 0x10c5411 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5412); }
 let mut pc: u32 = 0x10c5412;
 loop { match pc {
 0x10c5412 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c543c; continue; }
  pc = 0x10c541a; continue;
 }
 0x10c541a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5412; continue; }
  pc = 0x10c543c; continue;
 }
 0x10c543c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c543d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c543d); }
 let mut pc: u32 = 0x10c543d;
 loop { match pc {
 0x10c543d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5466; continue; }
  pc = 0x10c5445; continue;
 }
 0x10c5445 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c543d; continue; }
  pc = 0x10c5466; continue;
 }
 0x10c5466 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5467); }
 let mut pc: u32 = 0x10c5467;
 loop { match pc {
 0x10c5467 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5491; continue; }
  pc = 0x10c546f; continue;
 }
 0x10c546f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5467; continue; }
  pc = 0x10c5491; continue;
 }
 0x10c5491 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5492(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5492); }
 let mut pc: u32 = 0x10c5492;
 loop { match pc {
 0x10c5492 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c54bb; continue; }
  pc = 0x10c549a; continue;
 }
 0x10c549a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5492; continue; }
  pc = 0x10c54bb; continue;
 }
 0x10c54bb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c54bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c54bc); }
 let mut pc: u32 = 0x10c54bc;
 loop { match pc {
 0x10c54bc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c54f5; continue; }
  pc = 0x10c54c4; continue;
 }
 0x10c54c4 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c54bc; continue; }
  pc = 0x10c54f5; continue;
 }
 0x10c54f5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c54f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c54f6); }
 let mut pc: u32 = 0x10c54f6;
 loop { match pc {
 0x10c54f6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5538; continue; }
  pc = 0x10c54fe; continue;
 }
 0x10c54fe => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c54f6; continue; }
  pc = 0x10c5538; continue;
 }
 0x10c5538 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5539(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5539); }
 let mut pc: u32 = 0x10c5539;
 loop { match pc {
 0x10c5539 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c557c; continue; }
  pc = 0x10c5541; continue;
 }
 0x10c5541 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5539; continue; }
  pc = 0x10c557c; continue;
 }
 0x10c557c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c557d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c557d); }
 let mut pc: u32 = 0x10c557d;
 loop { match pc {
 0x10c557d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c55bf; continue; }
  pc = 0x10c5585; continue;
 }
 0x10c5585 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c557d; continue; }
  pc = 0x10c55bf; continue;
 }
 0x10c55bf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c55c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c55c0); }
 let mut pc: u32 = 0x10c55c0;
 loop { match pc {
 0x10c55c0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5603; continue; }
  pc = 0x10c55c8; continue;
 }
 0x10c55c8 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c55c0; continue; }
  pc = 0x10c5603; continue;
 }
 0x10c5603 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5604(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5604); }
 let mut pc: u32 = 0x10c5604;
 loop { match pc {
 0x10c5604 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5628; continue; }
  pc = 0x10c560c; continue;
 }
 0x10c560c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5604; continue; }
  pc = 0x10c5628; continue;
 }
 0x10c5628 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5629(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5629); }
 let mut pc: u32 = 0x10c5629;
 loop { match pc {
 0x10c5629 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c565d; continue; }
  pc = 0x10c5631; continue;
 }
 0x10c5631 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5629; continue; }
  pc = 0x10c565d; continue;
 }
 0x10c565d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c565e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c565e); }
 let mut pc: u32 = 0x10c565e;
 loop { match pc {
 0x10c565e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5695; continue; }
  pc = 0x10c5666; continue;
 }
 0x10c5666 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c565e; continue; }
  pc = 0x10c5695; continue;
 }
 0x10c5695 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5696(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5696); }
 let mut pc: u32 = 0x10c5696;
 loop { match pc {
 0x10c5696 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c56ce; continue; }
  pc = 0x10c569e; continue;
 }
 0x10c569e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5696; continue; }
  pc = 0x10c56ce; continue;
 }
 0x10c56ce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c56cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c56cf); }
 let mut pc: u32 = 0x10c56cf;
 loop { match pc {
 0x10c56cf => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5706; continue; }
  pc = 0x10c56d7; continue;
 }
 0x10c56d7 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c56cf; continue; }
  pc = 0x10c5706; continue;
 }
 0x10c5706 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5707(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5707); }
 let mut pc: u32 = 0x10c5707;
 loop { match pc {
 0x10c5707 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c573f; continue; }
  pc = 0x10c570f; continue;
 }
 0x10c570f => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5707; continue; }
  pc = 0x10c573f; continue;
 }
 0x10c573f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5740); }
 let mut pc: u32 = 0x10c5740;
 loop { match pc {
 0x10c5740 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5766; continue; }
  pc = 0x10c5748; continue;
 }
 0x10c5748 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5740; continue; }
  pc = 0x10c5766; continue;
 }
 0x10c5766 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5767(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5767); }
 let mut pc: u32 = 0x10c5767;
 loop { match pc {
 0x10c5767 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5797; continue; }
  pc = 0x10c576f; continue;
 }
 0x10c576f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x117cc68u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5767; continue; }
  pc = 0x10c5797; continue;
 }
 0x10c5797 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5798(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5798); }
 let mut pc: u32 = 0x10c5798;
 loop { match pc {
 0x10c5798 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c57cf; continue; }
  pc = 0x10c57a0; continue;
 }
 0x10c57a0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5798; continue; }
  pc = 0x10c57cf; continue;
 }
 0x10c57cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c57d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c57d0); }
 let mut pc: u32 = 0x10c57d0;
 loop { match pc {
 0x10c57d0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5808; continue; }
  pc = 0x10c57d8; continue;
 }
 0x10c57d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c57d0; continue; }
  pc = 0x10c5808; continue;
 }
 0x10c5808 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5809(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5809); }
 let mut pc: u32 = 0x10c5809;
 loop { match pc {
 0x10c5809 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5840; continue; }
  pc = 0x10c5811; continue;
 }
 0x10c5811 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5809; continue; }
  pc = 0x10c5840; continue;
 }
 0x10c5840 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5841(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5841); }
 let mut pc: u32 = 0x10c5841;
 loop { match pc {
 0x10c5841 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5879; continue; }
  pc = 0x10c5849; continue;
 }
 0x10c5849 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5841; continue; }
  pc = 0x10c5879; continue;
 }
 0x10c5879 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c587a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c587a); }
 let mut pc: u32 = 0x10c587a;
 loop { match pc {
 0x10c587a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c58b1; continue; }
  pc = 0x10c5882; continue;
 }
 0x10c5882 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c587a; continue; }
  pc = 0x10c58b1; continue;
 }
 0x10c58b1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c58b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c58b2); }
 let mut pc: u32 = 0x10c58b2;
 loop { match pc {
 0x10c58b2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c58f9; continue; }
  pc = 0x10c58ba; continue;
 }
 0x10c58ba => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c58b2; continue; }
  pc = 0x10c58f9; continue;
 }
 0x10c58f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c58fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c58fa); }
 let mut pc: u32 = 0x10c58fa;
 loop { match pc {
 0x10c58fa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c594a; continue; }
  pc = 0x10c5902; continue;
 }
 0x10c5902 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c58fa; continue; }
  pc = 0x10c594a; continue;
 }
 0x10c594a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c594b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c594b); }
 let mut pc: u32 = 0x10c594b;
 loop { match pc {
 0x10c594b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c599c; continue; }
  pc = 0x10c5953; continue;
 }
 0x10c5953 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c594b; continue; }
  pc = 0x10c599c; continue;
 }
 0x10c599c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c599d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c599d); }
 let mut pc: u32 = 0x10c599d;
 loop { match pc {
 0x10c599d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c59ed; continue; }
  pc = 0x10c59a5; continue;
 }
 0x10c59a5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c599d; continue; }
  pc = 0x10c59ed; continue;
 }
 0x10c59ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c59ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c59ee); }
 let mut pc: u32 = 0x10c59ee;
 loop { match pc {
 0x10c59ee => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5a3f; continue; }
  pc = 0x10c59f6; continue;
 }
 0x10c59f6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c59ee; continue; }
  pc = 0x10c5a3f; continue;
 }
 0x10c5a3f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5a40); }
 let mut pc: u32 = 0x10c5a40;
 loop { match pc {
 0x10c5a40 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5a72; continue; }
  pc = 0x10c5a48; continue;
 }
 0x10c5a48 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5a40; continue; }
  pc = 0x10c5a72; continue;
 }
 0x10c5a72 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5ab6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5ab6); }
 let mut pc: u32 = 0x10c5ab6;
 loop { match pc {
 0x10c5ab6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5afb; continue; }
  pc = 0x10c5abe; continue;
 }
 0x10c5abe => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5ab6; continue; }
  pc = 0x10c5afb; continue;
 }
 0x10c5afb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5afc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5afc); }
 let mut pc: u32 = 0x10c5afc;
 loop { match pc {
 0x10c5afc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5b42; continue; }
  pc = 0x10c5b04; continue;
 }
 0x10c5b04 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5afc; continue; }
  pc = 0x10c5b42; continue;
 }
 0x10c5b42 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5b43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5b43); }
 let mut pc: u32 = 0x10c5b43;
 loop { match pc {
 0x10c5b43 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5b88; continue; }
  pc = 0x10c5b4b; continue;
 }
 0x10c5b4b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5b43; continue; }
  pc = 0x10c5b88; continue;
 }
 0x10c5b88 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010c5b89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10c5b89); }
 let mut pc: u32 = 0x10c5b89;
 loop { match pc {
 0x10c5b89 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x117cc58u32)); }
  if !cpu.cf { pc = 0x10c5bcf; continue; }
  pc = 0x10c5b91; continue;
 }
 0x10c5b91 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x117cc64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x117cc68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x117cc60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x10c5b89; continue; }
  pc = 0x10c5bcf; continue;
 }
 0x10c5bcf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
