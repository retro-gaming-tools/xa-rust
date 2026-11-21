#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_06110c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6110c24); }
 let mut pc: u32 = 0x6110c24;
 loop { match pc {
 0x6110c24 => {
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x11u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)).wrapping_add(cpu.bases[3])), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0xcu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  cpu.call(0x6110c34); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0611158c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x611158c); }
 let mut pc: u32 = 0x611158c;
 loop { match pc {
 0x611158c => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x61115a0; continue; }
  pc = 0x611159d; continue;
 }
 0x611159d => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x61115a0; continue;
 }
 0x61115a0 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { pc = 0x61115a8; continue; }
  pc = 0x61115a6; continue;
 }
 0x61115a6 => {
  { cpu.stos(2, false); }
  pc = 0x61115a8; continue;
 }
 0x61115a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_061115a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x61115a9); }
 let mut pc: u32 = 0x61115a9;
 loop { match pc {
 0x61115a9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x61115b0; continue;
 }
 0x61115b0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x61115b2; continue;
 }
 0x61115b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61115b2; continue; }
  pc = 0x61115be; continue;
 }
 0x61115be => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61115b0; continue; }
  pc = 0x61115c3; continue;
 }
 0x61115c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_061117df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x61117df); }
 let mut pc: u32 = 0x61117df;
 loop { match pc {
 0x61117df => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x6111817; continue; }
  pc = 0x61117f5; continue;
 }
 0x61117f5 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x6111807; continue; }
  pc = 0x61117f9; continue;
 }
 0x61117f9 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61117f9; continue; }
  pc = 0x6111804; continue;
 }
 0x6111804 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x6111807 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6111807; continue; }
  pc = 0x6111814; continue;
 }
 0x6111814 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x6111817 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x611182e; continue; }
  pc = 0x611181b; continue;
 }
 0x611181b => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x611181c; continue;
 }
 0x611181c => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x611181c; continue; }
  pc = 0x611182b; continue;
 }
 0x611182b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x611182e => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x611182e; continue; }
  pc = 0x611183b; continue;
 }
 0x611183b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0611183e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x611183e); }
 let mut pc: u32 = 0x611183e;
 loop { match pc {
 0x611183e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x62f1eb0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x62f1eacu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x6111a55; continue;
 }
 0x6111869 => {
  { let v: u32 = cpu.r32(0x62f1eacu32); cpu.ecx = v; }
  pc = 0x6111938; continue;
 }
 0x6111874 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x611187e; continue; }
  pc = 0x611187b; continue;
 }
 0x611187b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x611187e; continue;
 }
 0x611187e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x611188a; continue; }
  pc = 0x6111886; continue;
 }
 0x6111886 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x611188a; continue;
 }
 0x611188a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111896; continue; }
  pc = 0x6111892; continue;
 }
 0x6111892 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111896; continue;
 }
 0x6111896 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118a2; continue; }
  pc = 0x611189e; continue;
 }
 0x611189e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x61118a2; continue;
 }
 0x61118a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118ae; continue; }
  pc = 0x61118aa; continue;
 }
 0x61118aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x61118ae; continue;
 }
 0x61118ae => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118ba; continue; }
  pc = 0x61118b6; continue;
 }
 0x61118b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x61118ba; continue;
 }
 0x61118ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118c6; continue; }
  pc = 0x61118c2; continue;
 }
 0x61118c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x61118c6; continue;
 }
 0x61118c6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118d2; continue; }
  pc = 0x61118ce; continue;
 }
 0x61118ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x61118d2; continue;
 }
 0x61118d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118de; continue; }
  pc = 0x61118da; continue;
 }
 0x61118da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x61118de; continue;
 }
 0x61118de => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118ea; continue; }
  pc = 0x61118e6; continue;
 }
 0x61118e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x61118ea; continue;
 }
 0x61118ea => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61118f6; continue; }
  pc = 0x61118f2; continue;
 }
 0x61118f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x61118f6; continue;
 }
 0x61118f6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111902; continue; }
  pc = 0x61118fe; continue;
 }
 0x61118fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111902; continue;
 }
 0x6111902 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x611190e; continue; }
  pc = 0x611190a; continue;
 }
 0x611190a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x611190e; continue;
 }
 0x611190e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x611191a; continue; }
  pc = 0x6111916; continue;
 }
 0x6111916 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x611191a; continue;
 }
 0x611191a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111926; continue; }
  pc = 0x6111922; continue;
 }
 0x6111922 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x6111926; continue;
 }
 0x6111926 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111932; continue; }
  pc = 0x611192e; continue;
 }
 0x611192e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x6111932; continue;
 }
 0x6111932 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x6111938; continue;
 }
 0x6111938 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6111874; continue; }
  pc = 0x6111941; continue;
 }
 0x6111941 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x611194d; continue;
 }
 0x611194d => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111956; continue; }
  pc = 0x6111953; continue;
 }
 0x6111953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111956; continue;
 }
 0x6111956 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111962; continue; }
  pc = 0x611195e; continue;
 }
 0x611195e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111962; continue;
 }
 0x6111962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x611196e; continue;
 }
 0x611196e => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111977; continue; }
  pc = 0x6111974; continue;
 }
 0x6111974 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111977; continue;
 }
 0x6111977 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111983; continue; }
  pc = 0x611197f; continue;
 }
 0x611197f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111983; continue;
 }
 0x6111983 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x611198f; continue;
 }
 0x611198f => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111998; continue; }
  pc = 0x6111995; continue;
 }
 0x6111995 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111998; continue;
 }
 0x6111998 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119a4; continue; }
  pc = 0x61119a0; continue;
 }
 0x61119a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119a4; continue;
 }
 0x61119a4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x61119b0; continue;
 }
 0x61119b0 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119b9; continue; }
  pc = 0x61119b6; continue;
 }
 0x61119b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119b9; continue;
 }
 0x61119b9 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119c5; continue; }
  pc = 0x61119c1; continue;
 }
 0x61119c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119c5; continue;
 }
 0x61119c5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x61119cd; continue;
 }
 0x61119cd => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119d6; continue; }
  pc = 0x61119d3; continue;
 }
 0x61119d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119d6; continue;
 }
 0x61119d6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119e2; continue; }
  pc = 0x61119de; continue;
 }
 0x61119de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119e2; continue;
 }
 0x61119e2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x61119ea; continue;
 }
 0x61119ea => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119f3; continue; }
  pc = 0x61119f0; continue;
 }
 0x61119f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119f3; continue;
 }
 0x61119f3 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x61119ff; continue; }
  pc = 0x61119fb; continue;
 }
 0x61119fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x61119ff; continue;
 }
 0x61119ff => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x6111a07; continue;
 }
 0x6111a07 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111a10; continue; }
  pc = 0x6111a0d; continue;
 }
 0x6111a0d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111a10; continue;
 }
 0x6111a10 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111a1c; continue; }
  pc = 0x6111a18; continue;
 }
 0x6111a18 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111a1c; continue;
 }
 0x6111a1c => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x6111a3c; continue; }
  pc = 0x6111a24; continue;
 }
 0x6111a24 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111a2d; continue; }
  pc = 0x6111a2a; continue;
 }
 0x6111a2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111a2d; continue;
 }
 0x6111a2d => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111a39; continue; }
  pc = 0x6111a35; continue;
 }
 0x6111a35 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111a39; continue;
 }
 0x6111a39 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x6111a3c; continue;
 }
 0x6111a3c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x6111a51; continue; }
  pc = 0x6111a44; continue;
 }
 0x6111a44 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x6111a4e; continue; }
  pc = 0x6111a4b; continue;
 }
 0x6111a4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x6111a4e; continue;
 }
 0x6111a4e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x6111a51; continue;
 }
 0x6111a51 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x6111a55; continue;
 }
 0x6111a55 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x62f1eb0u32)); cpu.w32(0x62f1eb0u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x6111869; continue; }
  pc = 0x6111a61; continue;
 }
 0x6111a61 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06111a65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6111a65); }
 let mut pc: u32 = 0x6111a65;
 loop { match pc {
 0x6111a65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x62f1eb8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x62f1eb4u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x62f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x6111a84; continue;
 }
 0x6111a84 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.sf { pc = 0x6111a93; continue; }
  pc = 0x6111a88; continue;
 }
 0x6111a88 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x62f1eb4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  pc = 0x6111a93; continue;
 }
 0x6111a93 => {
  { let v: u32 = cpu.r32(0x62f1eb8u32); cpu.edi = v; }
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x62f1eb4u32)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x6111a84; continue; }
  pc = 0x6111adf; continue;
 }
 0x6111adf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06111ae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6111ae1); }
 let mut pc: u32 = 0x6111ae1;
 loop { match pc {
 0x6111ae1 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6111ae1; continue; }
  pc = 0x6111aeb; continue;
 }
 0x6111aeb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06120000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6120000); }
 let mut pc: u32 = 0x6120000;
 loop { match pc {
 0x6120000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x625192cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x6251928u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612018a; continue; }
  pc = 0x6120023; continue;
 }
 0x6120023 => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612006d; continue; }
  pc = 0x612002f; continue;
 }
 0x612002f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120064; continue; }
  pc = 0x6120053; continue;
 }
 0x6120053 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120064; continue;
 }
 0x6120064 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612006d; continue;
 }
 0x612006d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x61200a2; continue;
 }
 0x6120072 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61200a2; continue;
 }
 0x61200a2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ecx = v; }
  pc = 0x61200ab; continue;
 }
 0x61200ab => {
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
  if !cpu.zf { cpu.poll(); pc = 0x61200ab; continue; }
  pc = 0x61200ce; continue;
 }
 0x61200ce => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x6120072; continue; }
  pc = 0x61200d8; continue;
 }
 0x61200d8 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x6120114; continue;
 }
 0x6120114 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x6120114; continue; }
  pc = 0x6120137; continue;
 }
 0x6120137 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120185; continue; }
  pc = 0x6120147; continue;
 }
 0x6120147 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612017c; continue; }
  pc = 0x612016b; continue;
 }
 0x612016b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612017c; continue;
 }
 0x612017c => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120185; continue;
 }
 0x6120185 => {
  pc = 0x612027a; continue;
 }
 0x612018a => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61201d4; continue; }
  pc = 0x6120196; continue;
 }
 0x6120196 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61201cb; continue; }
  pc = 0x61201ba; continue;
 }
 0x61201ba => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61201cb; continue;
 }
 0x61201cb => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61201d4; continue;
 }
 0x61201d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x61201f7; continue;
 }
 0x61201d9 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { cpu.interrupt(0x10); }
  pc = 0x61201f7; continue;
 }
 0x61201f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ecx = v; }
  pc = 0x6120200; continue;
 }
 0x6120200 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x6120200; continue; }
  pc = 0x6120223; continue;
 }
 0x6120223 => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x61201d9; continue; }
  pc = 0x612022d; continue;
 }
 0x612022d => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x6120257; continue;
 }
 0x6120257 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x6120257; continue; }
  pc = 0x612027a; continue;
 }
 0x612027a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612027c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612027c); }
 let mut pc: u32 = 0x612027c;
 loop { match pc {
 0x612027c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61202f1; continue; }
  pc = 0x61202b3; continue;
 }
 0x61202b3 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61202e8; continue; }
  pc = 0x61202d7; continue;
 }
 0x61202d7 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61202e8; continue;
 }
 0x61202e8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61202f1; continue;
 }
 0x61202f1 => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x612032f; continue;
 }
 0x612032f => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x6120350; continue; }
  pc = 0x6120337; continue;
 }
 0x6120337 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612032f; continue; }
  pc = 0x612034b; continue;
 }
 0x612034b => {
  pc = 0x6120474; continue;
 }
 0x6120350 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x61203fb; continue; }
  pc = 0x6120361; continue;
 }
 0x6120361 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61203c6; continue; }
  pc = 0x6120388; continue;
 }
 0x6120388 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61203bd; continue; }
  pc = 0x61203ac; continue;
 }
 0x61203ac => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61203bd; continue;
 }
 0x61203bd => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61203c6; continue;
 }
 0x61203c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612032f; continue; }
  pc = 0x61203f9; continue;
 }
 0x61203f9 => {
  pc = 0x6120474; continue;
 }
 0x61203fb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612045d; continue; }
  pc = 0x612041f; continue;
 }
 0x612041f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120454; continue; }
  pc = 0x6120443; continue;
 }
 0x6120443 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120454; continue;
 }
 0x6120454 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612045d; continue;
 }
 0x612045d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612032f; continue; }
  pc = 0x6120474; continue;
 }
 0x6120474 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06120477(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6120477); }
 let mut pc: u32 = 0x6120477;
 loop { match pc {
 0x6120477 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61204ec; continue; }
  pc = 0x61204ae; continue;
 }
 0x61204ae => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61204e3; continue; }
  pc = 0x61204d2; continue;
 }
 0x61204d2 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61204e3; continue;
 }
 0x61204e3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61204ec; continue;
 }
 0x61204ec => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x612052a; continue;
 }
 0x612052a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x612054b; continue; }
  pc = 0x6120532; continue;
 }
 0x6120532 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612052a; continue; }
  pc = 0x6120546; continue;
 }
 0x6120546 => {
  pc = 0x6120671; continue;
 }
 0x612054b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x61205f8; continue; }
  pc = 0x612055c; continue;
 }
 0x612055c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61205c1; continue; }
  pc = 0x6120583; continue;
 }
 0x6120583 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61205b8; continue; }
  pc = 0x61205a7; continue;
 }
 0x61205a7 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61205b8; continue;
 }
 0x61205b8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61205c1; continue;
 }
 0x61205c1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612052a; continue; }
  pc = 0x61205f6; continue;
 }
 0x61205f6 => {
  pc = 0x6120671; continue;
 }
 0x61205f8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612065a; continue; }
  pc = 0x612061c; continue;
 }
 0x612061c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120651; continue; }
  pc = 0x6120640; continue;
 }
 0x6120640 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120651; continue;
 }
 0x6120651 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612065a; continue;
 }
 0x612065a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612052a; continue; }
  pc = 0x6120671; continue;
 }
 0x6120671 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06120674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6120674); }
 let mut pc: u32 = 0x6120674;
 loop { match pc {
 0x6120674 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61206c4; continue; }
  pc = 0x6120686; continue;
 }
 0x6120686 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61206bb; continue; }
  pc = 0x61206aa; continue;
 }
 0x61206aa => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61206bb; continue;
 }
 0x61206bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61206c4; continue;
 }
 0x61206c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x6300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x61206e6; continue; }
  pc = 0x61206db; continue;
 }
 0x61206db => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x61206e6; continue;
 }
 0x61206e6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61206fd; continue;
 }
 0x61206ed => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61206fd; continue;
 }
 0x61206fd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61206ed; continue; }
  pc = 0x6120700; continue;
 }
 0x6120700 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61206e6; continue; }
  pc = 0x6120709; continue;
 }
 0x6120709 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120720; continue;
 }
 0x6120710 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120720; continue;
 }
 0x6120720 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120710; continue; }
  pc = 0x6120723; continue;
 }
 0x6120723 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120791; continue; }
  pc = 0x6120753; continue;
 }
 0x6120753 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120788; continue; }
  pc = 0x6120777; continue;
 }
 0x6120777 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120788; continue;
 }
 0x6120788 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120791; continue;
 }
 0x6120791 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x61207af; continue;
 }
 0x61207af => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61207c6; continue;
 }
 0x61207b6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61207c6; continue;
 }
 0x61207c6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61207b6; continue; }
  pc = 0x61207c9; continue;
 }
 0x61207c9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61207af; continue; }
  pc = 0x61207d2; continue;
 }
 0x61207d2 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x61207e9; continue;
 }
 0x61207d9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61207e9; continue;
 }
 0x61207e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61207d9; continue; }
  pc = 0x61207ec; continue;
 }
 0x61207ec => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120841; continue; }
  pc = 0x6120803; continue;
 }
 0x6120803 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120838; continue; }
  pc = 0x6120827; continue;
 }
 0x6120827 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120838; continue;
 }
 0x6120838 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120841; continue;
 }
 0x6120841 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x612085e; continue;
 }
 0x612084e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612085e; continue;
 }
 0x612085e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612084e; continue; }
  pc = 0x6120861; continue;
 }
 0x6120861 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x612087f; continue;
 }
 0x612087f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120896; continue;
 }
 0x6120886 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120896; continue;
 }
 0x6120896 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120886; continue; }
  pc = 0x6120899; continue;
 }
 0x6120899 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612087f; continue; }
  pc = 0x61208a2; continue;
 }
 0x61208a2 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61208b9; continue;
 }
 0x61208a9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61208b9; continue;
 }
 0x61208b9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61208a9; continue; }
  pc = 0x61208bc; continue;
 }
 0x61208bc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612092a; continue; }
  pc = 0x61208ec; continue;
 }
 0x61208ec => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120921; continue; }
  pc = 0x6120910; continue;
 }
 0x6120910 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120921; continue;
 }
 0x6120921 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612092a; continue;
 }
 0x612092a => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6120948; continue;
 }
 0x6120948 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x612095f; continue;
 }
 0x612094f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612095f; continue;
 }
 0x612095f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612094f; continue; }
  pc = 0x6120962; continue;
 }
 0x6120962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120948; continue; }
  pc = 0x612096b; continue;
 }
 0x612096b => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x6120982; continue;
 }
 0x6120972 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120982; continue;
 }
 0x6120982 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120972; continue; }
  pc = 0x6120985; continue;
 }
 0x6120985 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61209da; continue; }
  pc = 0x612099c; continue;
 }
 0x612099c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61209d1; continue; }
  pc = 0x61209c0; continue;
 }
 0x61209c0 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61209d1; continue;
 }
 0x61209d1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61209da; continue;
 }
 0x61209da => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x61209f7; continue;
 }
 0x61209e7 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61209f7; continue;
 }
 0x61209f7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61209e7; continue; }
  pc = 0x61209fa; continue;
 }
 0x61209fa => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6120a18; continue;
 }
 0x6120a18 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120a2f; continue;
 }
 0x6120a1f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120a2f; continue;
 }
 0x6120a2f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120a1f; continue; }
  pc = 0x6120a32; continue;
 }
 0x6120a32 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120a18; continue; }
  pc = 0x6120a3b; continue;
 }
 0x6120a3b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120a90; continue; }
  pc = 0x6120a52; continue;
 }
 0x6120a52 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120a87; continue; }
  pc = 0x6120a76; continue;
 }
 0x6120a76 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120a87; continue;
 }
 0x6120a87 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120a90; continue;
 }
 0x6120a90 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6120a9b; continue;
 }
 0x6120a9b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120ab2; continue;
 }
 0x6120aa2 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120ab2; continue;
 }
 0x6120ab2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120aa2; continue; }
  pc = 0x6120ab5; continue;
 }
 0x6120ab5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120a9b; continue; }
  pc = 0x6120abe; continue;
 }
 0x6120abe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120ad5; continue;
 }
 0x6120ac5 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120ad5; continue;
 }
 0x6120ad5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120ac5; continue; }
  pc = 0x6120ad8; continue;
 }
 0x6120ad8 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120b46; continue; }
  pc = 0x6120b08; continue;
 }
 0x6120b08 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120b3d; continue; }
  pc = 0x6120b2c; continue;
 }
 0x6120b2c => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120b3d; continue;
 }
 0x6120b3d => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120b46; continue;
 }
 0x6120b46 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6120b64; continue;
 }
 0x6120b64 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120b7b; continue;
 }
 0x6120b6b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120b7b; continue;
 }
 0x6120b7b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120b6b; continue; }
  pc = 0x6120b7e; continue;
 }
 0x6120b7e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120b64; continue; }
  pc = 0x6120b87; continue;
 }
 0x6120b87 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x6120b9e; continue;
 }
 0x6120b8e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120b9e; continue;
 }
 0x6120b9e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120b8e; continue; }
  pc = 0x6120ba1; continue;
 }
 0x6120ba1 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120bf6; continue; }
  pc = 0x6120bb8; continue;
 }
 0x6120bb8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120bed; continue; }
  pc = 0x6120bdc; continue;
 }
 0x6120bdc => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120bed; continue;
 }
 0x6120bed => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120bf6; continue;
 }
 0x6120bf6 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x6120c13; continue;
 }
 0x6120c03 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120c13; continue;
 }
 0x6120c13 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120c03; continue; }
  pc = 0x6120c16; continue;
 }
 0x6120c16 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6120d32; continue; }
  pc = 0x6120c3f; continue;
 }
 0x6120c3f => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x6120c44; continue;
 }
 0x6120c44 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120c5b; continue;
 }
 0x6120c4b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120c5b; continue;
 }
 0x6120c5b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120c4b; continue; }
  pc = 0x6120c5e; continue;
 }
 0x6120c5e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120c44; continue; }
  pc = 0x6120c67; continue;
 }
 0x6120c67 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120c7e; continue;
 }
 0x6120c6e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120c7e; continue;
 }
 0x6120c7e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120c6e; continue; }
  pc = 0x6120c81; continue;
 }
 0x6120c81 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120cef; continue; }
  pc = 0x6120cb1; continue;
 }
 0x6120cb1 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120ce6; continue; }
  pc = 0x6120cd5; continue;
 }
 0x6120cd5 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120ce6; continue;
 }
 0x6120ce6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120cef; continue;
 }
 0x6120cef => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x6120d0d; continue;
 }
 0x6120d0d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120d24; continue;
 }
 0x6120d14 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120d24; continue;
 }
 0x6120d24 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120d14; continue; }
  pc = 0x6120d27; continue;
 }
 0x6120d27 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120d0d; continue; }
  pc = 0x6120d30; continue;
 }
 0x6120d30 => {
  pc = 0x6120d5a; continue;
 }
 0x6120d32 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x6120d37; continue;
 }
 0x6120d37 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6120d4e; continue;
 }
 0x6120d3e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6120d4e; continue;
 }
 0x6120d4e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120d3e; continue; }
  pc = 0x6120d51; continue;
 }
 0x6120d51 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120d37; continue; }
  pc = 0x6120d5a; continue;
 }
 0x6120d5a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06120d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6120d5c); }
 let mut pc: u32 = 0x6120d5c;
 loop { match pc {
 0x6120d5c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6120d7b; continue; }
  pc = 0x6120d78; continue;
 }
 0x6120d78 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x6120d7b; continue;
 }
 0x6120d7b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120de5; continue; }
  pc = 0x6120da7; continue;
 }
 0x6120da7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120ddc; continue; }
  pc = 0x6120dcb; continue;
 }
 0x6120dcb => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120ddc; continue;
 }
 0x6120ddc => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120de5; continue;
 }
 0x6120de5 => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x6120e20; continue;
 }
 0x6120e20 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x6120e47; continue; }
  pc = 0x6120e28; continue;
 }
 0x6120e28 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6120e36; continue;
 }
 0x6120e2d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6120e36; continue;
 }
 0x6120e36 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e2d; continue; }
  pc = 0x6120e39; continue;
 }
 0x6120e39 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e20; continue; }
  pc = 0x6120e42; continue;
 }
 0x6120e42 => {
  pc = 0x612110c; continue;
 }
 0x6120e47 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x6120f13; continue; }
  pc = 0x6120e58; continue;
 }
 0x6120e58 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6120e66; continue;
 }
 0x6120e5d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6120e66; continue;
 }
 0x6120e66 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e5d; continue; }
  pc = 0x6120e69; continue;
 }
 0x6120e69 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120ec0; continue; }
  pc = 0x6120e82; continue;
 }
 0x6120e82 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120eb7; continue; }
  pc = 0x6120ea6; continue;
 }
 0x6120ea6 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120eb7; continue;
 }
 0x6120eb7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120ec0; continue;
 }
 0x6120ec0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6120ee0; continue;
 }
 0x6120ed7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6120ee0; continue;
 }
 0x6120ee0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120ed7; continue; }
  pc = 0x6120ee3; continue;
 }
 0x6120ee3 => {
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
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e20; continue; }
  pc = 0x6120f0e; continue;
 }
 0x6120f0e => {
  pc = 0x612110c; continue;
 }
 0x6120f13 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x612106f; continue; }
  pc = 0x6120f21; continue;
 }
 0x6120f21 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x6120feb; continue; }
  pc = 0x6120f2a; continue;
 }
 0x6120f2a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6120f38; continue;
 }
 0x6120f2f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6120f38; continue;
 }
 0x6120f38 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120f2f; continue; }
  pc = 0x6120f3b; continue;
 }
 0x6120f3b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6120fb2; continue; }
  pc = 0x6120f74; continue;
 }
 0x6120f74 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6120fa9; continue; }
  pc = 0x6120f98; continue;
 }
 0x6120f98 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6120fa9; continue;
 }
 0x6120fa9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6120fb2; continue;
 }
 0x6120fb2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e20; continue; }
  pc = 0x6120fe6; continue;
 }
 0x6120fe6 => {
  pc = 0x612110c; continue;
 }
 0x6120feb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6120ff9; continue;
 }
 0x6120ff0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6120ff9; continue;
 }
 0x6120ff9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120ff0; continue; }
  pc = 0x6120ffc; continue;
 }
 0x6120ffc => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121053; continue; }
  pc = 0x6121015; continue;
 }
 0x6121015 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612104a; continue; }
  pc = 0x6121039; continue;
 }
 0x6121039 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612104a; continue;
 }
 0x612104a => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121053; continue;
 }
 0x6121053 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e20; continue; }
  pc = 0x612106a; continue;
 }
 0x612106a => {
  pc = 0x612110c; continue;
 }
 0x612106f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x612107d; continue;
 }
 0x6121074 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x612107d; continue;
 }
 0x612107d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121074; continue; }
  pc = 0x6121080; continue;
 }
 0x6121080 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61210dd; continue; }
  pc = 0x612109f; continue;
 }
 0x612109f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61210d4; continue; }
  pc = 0x61210c3; continue;
 }
 0x61210c3 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61210d4; continue;
 }
 0x61210d4 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61210dd; continue;
 }
 0x61210dd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6120e20; continue; }
  pc = 0x612110c; continue;
 }
 0x612110c => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612110f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612110f); }
 let mut pc: u32 = 0x612110f;
 loop { match pc {
 0x612110f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x612112e; continue; }
  pc = 0x612112b; continue;
 }
 0x612112b => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x612112e; continue;
 }
 0x612112e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121198; continue; }
  pc = 0x612115a; continue;
 }
 0x612115a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612118f; continue; }
  pc = 0x612117e; continue;
 }
 0x612117e => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612118f; continue;
 }
 0x612118f => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121198; continue;
 }
 0x6121198 => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x61211d3; continue;
 }
 0x61211d3 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x61211f4; continue; }
  pc = 0x61211db; continue;
 }
 0x61211db => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61211e3; continue;
 }
 0x61211e0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61211e3; continue;
 }
 0x61211e3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211e0; continue; }
  pc = 0x61211e6; continue;
 }
 0x61211e6 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211d3; continue; }
  pc = 0x61211ef; continue;
 }
 0x61211ef => {
  pc = 0x612148b; continue;
 }
 0x61211f4 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x61212b0; continue; }
  pc = 0x6121205; continue;
 }
 0x6121205 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x612120d; continue;
 }
 0x612120a => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x612120d; continue;
 }
 0x612120d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612120a; continue; }
  pc = 0x6121210; continue;
 }
 0x6121210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121267; continue; }
  pc = 0x6121229; continue;
 }
 0x6121229 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612125e; continue; }
  pc = 0x612124d; continue;
 }
 0x612124d => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612125e; continue;
 }
 0x612125e => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121267; continue;
 }
 0x6121267 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6121281; continue;
 }
 0x612127e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6121281; continue;
 }
 0x6121281 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612127e; continue; }
  pc = 0x6121284; continue;
 }
 0x6121284 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121295; continue;
 }
 0x6121292 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6121295; continue;
 }
 0x6121295 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121292; continue; }
  pc = 0x6121298; continue;
 }
 0x6121298 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211d3; continue; }
  pc = 0x61212ab; continue;
 }
 0x61212ab => {
  pc = 0x612148b; continue;
 }
 0x61212b0 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x61213f8; continue; }
  pc = 0x61212be; continue;
 }
 0x61212be => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x612137a; continue; }
  pc = 0x61212c7; continue;
 }
 0x61212c7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61212cf; continue;
 }
 0x61212cc => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61212cf; continue;
 }
 0x61212cf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61212cc; continue; }
  pc = 0x61212d2; continue;
 }
 0x61212d2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x61212eb; continue;
 }
 0x61212e8 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61212eb; continue;
 }
 0x61212eb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61212e8; continue; }
  pc = 0x61212ee; continue;
 }
 0x61212ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121345; continue; }
  pc = 0x6121307; continue;
 }
 0x6121307 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612133c; continue; }
  pc = 0x612132b; continue;
 }
 0x612132b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612133c; continue;
 }
 0x612133c => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121345; continue;
 }
 0x6121345 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x612135f; continue;
 }
 0x612135c => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x612135f; continue;
 }
 0x612135f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612135c; continue; }
  pc = 0x6121362; continue;
 }
 0x6121362 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211d3; continue; }
  pc = 0x6121375; continue;
 }
 0x6121375 => {
  pc = 0x612148b; continue;
 }
 0x612137a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121382; continue;
 }
 0x612137f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6121382; continue;
 }
 0x6121382 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612137f; continue; }
  pc = 0x6121385; continue;
 }
 0x6121385 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61213dc; continue; }
  pc = 0x612139e; continue;
 }
 0x612139e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61213d3; continue; }
  pc = 0x61213c2; continue;
 }
 0x61213c2 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61213d3; continue;
 }
 0x61213d3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61213dc; continue;
 }
 0x61213dc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211d3; continue; }
  pc = 0x61213f3; continue;
 }
 0x61213f3 => {
  pc = 0x612148b; continue;
 }
 0x61213f8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121400; continue;
 }
 0x61213fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6121400; continue;
 }
 0x6121400 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61213fd; continue; }
  pc = 0x6121403; continue;
 }
 0x6121403 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121460; continue; }
  pc = 0x6121422; continue;
 }
 0x6121422 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121457; continue; }
  pc = 0x6121446; continue;
 }
 0x6121446 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121457; continue;
 }
 0x6121457 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121460; continue;
 }
 0x6121460 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x612147b; continue;
 }
 0x6121478 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x612147b; continue;
 }
 0x612147b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121478; continue; }
  pc = 0x612147e; continue;
 }
 0x612147e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61211d3; continue; }
  pc = 0x612148b; continue;
 }
 0x612148b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612148e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612148e); }
 let mut pc: u32 = 0x612148e;
 loop { match pc {
 0x612148e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61214de; continue; }
  pc = 0x61214a0; continue;
 }
 0x61214a0 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61214d5; continue; }
  pc = 0x61214c4; continue;
 }
 0x61214c4 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61214d5; continue;
 }
 0x61214d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61214de; continue;
 }
 0x61214de => {
  { let v: u32 = cpu.edi; cpu.w32(0x6300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6121500; continue; }
  pc = 0x61214f5; continue;
 }
 0x61214f5 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x6121500; continue;
 }
 0x6121500 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121523; continue;
 }
 0x6121507 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121523; continue;
 }
 0x6121523 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121507; continue; }
  pc = 0x6121526; continue;
 }
 0x6121526 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121500; continue; }
  pc = 0x612152f; continue;
 }
 0x612152f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121546; continue;
 }
 0x6121536 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121546; continue;
 }
 0x6121546 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121536; continue; }
  pc = 0x6121549; continue;
 }
 0x6121549 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x6121566; continue;
 }
 0x6121556 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121566; continue;
 }
 0x6121566 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121556; continue; }
  pc = 0x6121569; continue;
 }
 0x6121569 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61215be; continue; }
  pc = 0x6121580; continue;
 }
 0x6121580 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61215b5; continue; }
  pc = 0x61215a4; continue;
 }
 0x61215a4 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61215b5; continue;
 }
 0x61215b5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61215be; continue;
 }
 0x61215be => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x61215db; continue;
 }
 0x61215cb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61215db; continue;
 }
 0x61215db => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61215cb; continue; }
  pc = 0x61215de; continue;
 }
 0x61215de => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x61215e3; continue;
 }
 0x61215e3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121606; continue;
 }
 0x61215ea => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121606; continue;
 }
 0x6121606 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61215ea; continue; }
  pc = 0x6121609; continue;
 }
 0x6121609 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61215e3; continue; }
  pc = 0x6121612; continue;
 }
 0x6121612 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x6121629; continue;
 }
 0x6121619 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121629; continue;
 }
 0x6121629 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121619; continue; }
  pc = 0x612162c; continue;
 }
 0x612162c => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121681; continue; }
  pc = 0x6121643; continue;
 }
 0x6121643 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121678; continue; }
  pc = 0x6121667; continue;
 }
 0x6121667 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121678; continue;
 }
 0x6121678 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121681; continue;
 }
 0x6121681 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x612169e; continue;
 }
 0x612168e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612169e; continue;
 }
 0x612169e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612168e; continue; }
  pc = 0x61216a1; continue;
 }
 0x61216a1 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61216be; continue;
 }
 0x61216ae => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61216be; continue;
 }
 0x61216be => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61216ae; continue; }
  pc = 0x61216c1; continue;
 }
 0x61216c1 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x61216c6; continue;
 }
 0x61216c6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61216e9; continue;
 }
 0x61216cd => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61216e9; continue;
 }
 0x61216e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61216cd; continue; }
  pc = 0x61216ec; continue;
 }
 0x61216ec => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61216c6; continue; }
  pc = 0x61216f5; continue;
 }
 0x61216f5 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x612170c; continue;
 }
 0x61216fc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612170c; continue;
 }
 0x612170c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61216fc; continue; }
  pc = 0x612170f; continue;
 }
 0x612170f => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x612172c; continue;
 }
 0x612171c => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612172c; continue;
 }
 0x612172c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612171c; continue; }
  pc = 0x612172f; continue;
 }
 0x612172f => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121784; continue; }
  pc = 0x6121746; continue;
 }
 0x6121746 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612177b; continue; }
  pc = 0x612176a; continue;
 }
 0x612176a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612177b; continue;
 }
 0x612177b => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121784; continue;
 }
 0x6121784 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x61217a1; continue;
 }
 0x6121791 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61217a1; continue;
 }
 0x61217a1 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121791; continue; }
  pc = 0x61217a4; continue;
 }
 0x61217a4 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x61217a9; continue;
 }
 0x61217a9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61217cc; continue;
 }
 0x61217b0 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61217cc; continue;
 }
 0x61217cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61217b0; continue; }
  pc = 0x61217cf; continue;
 }
 0x61217cf => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61217a9; continue; }
  pc = 0x61217d8; continue;
 }
 0x61217d8 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x61217ef; continue;
 }
 0x61217df => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61217ef; continue;
 }
 0x61217ef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61217df; continue; }
  pc = 0x61217f2; continue;
 }
 0x61217f2 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121847; continue; }
  pc = 0x6121809; continue;
 }
 0x6121809 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612183e; continue; }
  pc = 0x612182d; continue;
 }
 0x612182d => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612183e; continue;
 }
 0x612183e => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121847; continue;
 }
 0x6121847 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x6121864; continue;
 }
 0x6121854 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121864; continue;
 }
 0x6121864 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121854; continue; }
  pc = 0x6121867; continue;
 }
 0x6121867 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121884; continue;
 }
 0x6121874 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121884; continue;
 }
 0x6121884 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121874; continue; }
  pc = 0x6121887; continue;
 }
 0x6121887 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x612188c; continue;
 }
 0x612188c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x61218af; continue;
 }
 0x6121893 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61218af; continue;
 }
 0x61218af => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121893; continue; }
  pc = 0x61218b2; continue;
 }
 0x61218b2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612188c; continue; }
  pc = 0x61218bb; continue;
 }
 0x61218bb => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121910; continue; }
  pc = 0x61218d2; continue;
 }
 0x61218d2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121907; continue; }
  pc = 0x61218f6; continue;
 }
 0x61218f6 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121907; continue;
 }
 0x6121907 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121910; continue;
 }
 0x6121910 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x612191b; continue;
 }
 0x612191b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x612193e; continue;
 }
 0x6121922 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x612193e; continue;
 }
 0x612193e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121922; continue; }
  pc = 0x6121941; continue;
 }
 0x6121941 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612191b; continue; }
  pc = 0x612194a; continue;
 }
 0x612194a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121961; continue;
 }
 0x6121951 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121961; continue;
 }
 0x6121961 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121951; continue; }
  pc = 0x6121964; continue;
 }
 0x6121964 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x6121981; continue;
 }
 0x6121971 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121981; continue;
 }
 0x6121981 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121971; continue; }
  pc = 0x6121984; continue;
 }
 0x6121984 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61219d9; continue; }
  pc = 0x612199b; continue;
 }
 0x612199b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61219d0; continue; }
  pc = 0x61219bf; continue;
 }
 0x61219bf => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61219d0; continue;
 }
 0x61219d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61219d9; continue;
 }
 0x61219d9 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x61219f6; continue;
 }
 0x61219e6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x61219f6; continue;
 }
 0x61219f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61219e6; continue; }
  pc = 0x61219f9; continue;
 }
 0x61219f9 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x61219fe; continue;
 }
 0x61219fe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121a21; continue;
 }
 0x6121a05 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121a21; continue;
 }
 0x6121a21 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121a05; continue; }
  pc = 0x6121a24; continue;
 }
 0x6121a24 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61219fe; continue; }
  pc = 0x6121a2d; continue;
 }
 0x6121a2d => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x6121a44; continue;
 }
 0x6121a34 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121a44; continue;
 }
 0x6121a44 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121a34; continue; }
  pc = 0x6121a47; continue;
 }
 0x6121a47 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121a9c; continue; }
  pc = 0x6121a5e; continue;
 }
 0x6121a5e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121a93; continue; }
  pc = 0x6121a82; continue;
 }
 0x6121a82 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121a93; continue;
 }
 0x6121a93 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121a9c; continue;
 }
 0x6121a9c => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x6121ab9; continue;
 }
 0x6121aa9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121ab9; continue;
 }
 0x6121ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121aa9; continue; }
  pc = 0x6121abc; continue;
 }
 0x6121abc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121ad9; continue;
 }
 0x6121ac9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121ad9; continue;
 }
 0x6121ad9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121ac9; continue; }
  pc = 0x6121adc; continue;
 }
 0x6121adc => {
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6121c05; continue; }
  pc = 0x6121aec; continue;
 }
 0x6121aec => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x6121af1; continue;
 }
 0x6121af1 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121b14; continue;
 }
 0x6121af8 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121b14; continue;
 }
 0x6121b14 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121af8; continue; }
  pc = 0x6121b17; continue;
 }
 0x6121b17 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121af1; continue; }
  pc = 0x6121b20; continue;
 }
 0x6121b20 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121b37; continue;
 }
 0x6121b27 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121b37; continue;
 }
 0x6121b37 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121b27; continue; }
  pc = 0x6121b3a; continue;
 }
 0x6121b3a => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x6121b57; continue;
 }
 0x6121b47 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121b57; continue;
 }
 0x6121b57 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121b47; continue; }
  pc = 0x6121b5a; continue;
 }
 0x6121b5a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121baf; continue; }
  pc = 0x6121b71; continue;
 }
 0x6121b71 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121ba6; continue; }
  pc = 0x6121b95; continue;
 }
 0x6121b95 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121ba6; continue;
 }
 0x6121ba6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121baf; continue;
 }
 0x6121baf => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x6121bcc; continue;
 }
 0x6121bbc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121bcc; continue;
 }
 0x6121bcc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121bbc; continue; }
  pc = 0x6121bcf; continue;
 }
 0x6121bcf => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x6121bd4; continue;
 }
 0x6121bd4 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121bf7; continue;
 }
 0x6121bdb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121bf7; continue;
 }
 0x6121bf7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121bdb; continue; }
  pc = 0x6121bfa; continue;
 }
 0x6121bfa => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121bd4; continue; }
  pc = 0x6121c03; continue;
 }
 0x6121c03 => {
  pc = 0x6121c39; continue;
 }
 0x6121c05 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x6121c0a; continue;
 }
 0x6121c0a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x6121c2d; continue;
 }
 0x6121c11 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6121c2d; continue;
 }
 0x6121c2d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121c11; continue; }
  pc = 0x6121c30; continue;
 }
 0x6121c30 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121c0a; continue; }
  pc = 0x6121c39; continue;
 }
 0x6121c39 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06121c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6121c3b); }
 let mut pc: u32 = 0x6121c3b;
 loop { match pc {
 0x6121c3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6121c5a; continue; }
  pc = 0x6121c57; continue;
 }
 0x6121c57 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x6121c5a; continue;
 }
 0x6121c5a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121cc4; continue; }
  pc = 0x6121c86; continue;
 }
 0x6121c86 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121cbb; continue; }
  pc = 0x6121caa; continue;
 }
 0x6121caa => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121cbb; continue;
 }
 0x6121cbb => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121cc4; continue;
 }
 0x6121cc4 => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x6121cff; continue;
 }
 0x6121cff => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x6121d2c; continue; }
  pc = 0x6121d07; continue;
 }
 0x6121d07 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121d1b; continue;
 }
 0x6121d0c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x6121d1b; continue;
 }
 0x6121d1b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121d0c; continue; }
  pc = 0x6121d1e; continue;
 }
 0x6121d1e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121cff; continue; }
  pc = 0x6121d27; continue;
 }
 0x6121d27 => {
  pc = 0x6121fff; continue;
 }
 0x6121d2c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x6121dfa; continue; }
  pc = 0x6121d3d; continue;
 }
 0x6121d3d => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6121d4b; continue;
 }
 0x6121d42 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121d4b; continue;
 }
 0x6121d4b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121d42; continue; }
  pc = 0x6121d4e; continue;
 }
 0x6121d4e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121da5; continue; }
  pc = 0x6121d67; continue;
 }
 0x6121d67 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121d9c; continue; }
  pc = 0x6121d8b; continue;
 }
 0x6121d8b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121d9c; continue;
 }
 0x6121d9c => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121da5; continue;
 }
 0x6121da5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6121dc5; continue;
 }
 0x6121dbc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121dc5; continue;
 }
 0x6121dc5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121dbc; continue; }
  pc = 0x6121dc8; continue;
 }
 0x6121dc8 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121ddf; continue;
 }
 0x6121dd6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121ddf; continue;
 }
 0x6121ddf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121dd6; continue; }
  pc = 0x6121de2; continue;
 }
 0x6121de2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121cff; continue; }
  pc = 0x6121df5; continue;
 }
 0x6121df5 => {
  pc = 0x6121fff; continue;
 }
 0x6121dfa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x6121f60; continue; }
  pc = 0x6121e08; continue;
 }
 0x6121e08 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x6121ed6; continue; }
  pc = 0x6121e11; continue;
 }
 0x6121e11 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121e1f; continue;
 }
 0x6121e16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121e1f; continue;
 }
 0x6121e1f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121e16; continue; }
  pc = 0x6121e22; continue;
 }
 0x6121e22 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6121e41; continue;
 }
 0x6121e38 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121e41; continue;
 }
 0x6121e41 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121e38; continue; }
  pc = 0x6121e44; continue;
 }
 0x6121e44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121e9b; continue; }
  pc = 0x6121e5d; continue;
 }
 0x6121e5d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121e92; continue; }
  pc = 0x6121e81; continue;
 }
 0x6121e81 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121e92; continue;
 }
 0x6121e92 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121e9b; continue;
 }
 0x6121e9b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6121ebb; continue;
 }
 0x6121eb2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121ebb; continue;
 }
 0x6121ebb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121eb2; continue; }
  pc = 0x6121ebe; continue;
 }
 0x6121ebe => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121cff; continue; }
  pc = 0x6121ed1; continue;
 }
 0x6121ed1 => {
  pc = 0x6121fff; continue;
 }
 0x6121ed6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121eea; continue;
 }
 0x6121edb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x6121eea; continue;
 }
 0x6121eea => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121edb; continue; }
  pc = 0x6121eed; continue;
 }
 0x6121eed => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121f44; continue; }
  pc = 0x6121f06; continue;
 }
 0x6121f06 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121f3b; continue; }
  pc = 0x6121f2a; continue;
 }
 0x6121f2a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121f3b; continue;
 }
 0x6121f3b => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121f44; continue;
 }
 0x6121f44 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121cff; continue; }
  pc = 0x6121f5b; continue;
 }
 0x6121f5b => {
  pc = 0x6121fff; continue;
 }
 0x6121f60 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121f6e; continue;
 }
 0x6121f65 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121f6e; continue;
 }
 0x6121f6e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121f65; continue; }
  pc = 0x6121f71; continue;
 }
 0x6121f71 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6121fce; continue; }
  pc = 0x6121f90; continue;
 }
 0x6121f90 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6121fc5; continue; }
  pc = 0x6121fb4; continue;
 }
 0x6121fb4 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6121fc5; continue;
 }
 0x6121fc5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6121fce; continue;
 }
 0x6121fce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6121fef; continue;
 }
 0x6121fe6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x6121fef; continue;
 }
 0x6121fef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121fe6; continue; }
  pc = 0x6121ff2; continue;
 }
 0x6121ff2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6121cff; continue; }
  pc = 0x6121fff; continue;
 }
 0x6121fff => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06122002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6122002); }
 let mut pc: u32 = 0x6122002;
 loop { match pc {
 0x6122002 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6122021; continue; }
  pc = 0x612201e; continue;
 }
 0x612201e => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x6122021; continue;
 }
 0x6122021 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612208b; continue; }
  pc = 0x612204d; continue;
 }
 0x612204d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6122082; continue; }
  pc = 0x6122071; continue;
 }
 0x6122071 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6122082; continue;
 }
 0x6122082 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612208b; continue;
 }
 0x612208b => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x61220c6; continue;
 }
 0x61220c6 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x61220e7; continue; }
  pc = 0x61220ce; continue;
 }
 0x61220ce => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61220d6; continue;
 }
 0x61220d3 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61220d6; continue;
 }
 0x61220d6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220d3; continue; }
  pc = 0x61220d9; continue;
 }
 0x61220d9 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220c6; continue; }
  pc = 0x61220e2; continue;
 }
 0x61220e2 => {
  pc = 0x612237e; continue;
 }
 0x61220e7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x61221a3; continue; }
  pc = 0x61220f8; continue;
 }
 0x61220f8 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6122100; continue;
 }
 0x61220fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6122100; continue;
 }
 0x6122100 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220fd; continue; }
  pc = 0x6122103; continue;
 }
 0x6122103 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612215a; continue; }
  pc = 0x612211c; continue;
 }
 0x612211c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6122151; continue; }
  pc = 0x6122140; continue;
 }
 0x6122140 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6122151; continue;
 }
 0x6122151 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612215a; continue;
 }
 0x612215a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6122174; continue;
 }
 0x6122171 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6122174; continue;
 }
 0x6122174 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122171; continue; }
  pc = 0x6122177; continue;
 }
 0x6122177 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6122188; continue;
 }
 0x6122185 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6122188; continue;
 }
 0x6122188 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122185; continue; }
  pc = 0x612218b; continue;
 }
 0x612218b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220c6; continue; }
  pc = 0x612219e; continue;
 }
 0x612219e => {
  pc = 0x612237e; continue;
 }
 0x61221a3 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x61222eb; continue; }
  pc = 0x61221b1; continue;
 }
 0x61221b1 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x612226d; continue; }
  pc = 0x61221ba; continue;
 }
 0x61221ba => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61221c2; continue;
 }
 0x61221bf => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61221c2; continue;
 }
 0x61221c2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61221bf; continue; }
  pc = 0x61221c5; continue;
 }
 0x61221c5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x61221de; continue;
 }
 0x61221db => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61221de; continue;
 }
 0x61221de => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61221db; continue; }
  pc = 0x61221e1; continue;
 }
 0x61221e1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6122238; continue; }
  pc = 0x61221fa; continue;
 }
 0x61221fa => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612222f; continue; }
  pc = 0x612221e; continue;
 }
 0x612221e => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612222f; continue;
 }
 0x612222f => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6122238; continue;
 }
 0x6122238 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6122252; continue;
 }
 0x612224f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6122252; continue;
 }
 0x6122252 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612224f; continue; }
  pc = 0x6122255; continue;
 }
 0x6122255 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220c6; continue; }
  pc = 0x6122268; continue;
 }
 0x6122268 => {
  pc = 0x612237e; continue;
 }
 0x612226d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6122275; continue;
 }
 0x6122272 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6122275; continue;
 }
 0x6122275 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122272; continue; }
  pc = 0x6122278; continue;
 }
 0x6122278 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61222cf; continue; }
  pc = 0x6122291; continue;
 }
 0x6122291 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61222c6; continue; }
  pc = 0x61222b5; continue;
 }
 0x61222b5 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61222c6; continue;
 }
 0x61222c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61222cf; continue;
 }
 0x61222cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220c6; continue; }
  pc = 0x61222e6; continue;
 }
 0x61222e6 => {
  pc = 0x612237e; continue;
 }
 0x61222eb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61222f3; continue;
 }
 0x61222f0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x61222f3; continue;
 }
 0x61222f3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61222f0; continue; }
  pc = 0x61222f6; continue;
 }
 0x61222f6 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6122353; continue; }
  pc = 0x6122315; continue;
 }
 0x6122315 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612234a; continue; }
  pc = 0x6122339; continue;
 }
 0x6122339 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612234a; continue;
 }
 0x612234a => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6122353; continue;
 }
 0x6122353 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x612236e; continue;
 }
 0x612236b => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x612236e; continue;
 }
 0x612236e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612236b; continue; }
  pc = 0x6122371; continue;
 }
 0x6122371 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61220c6; continue; }
  pc = 0x612237e; continue;
 }
 0x612237e => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06122381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6122381); }
 let mut pc: u32 = 0x6122381;
 loop { match pc {
 0x6122381 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61223d1; continue; }
  pc = 0x6122393; continue;
 }
 0x6122393 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61223c8; continue; }
  pc = 0x61223b7; continue;
 }
 0x61223b7 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61223c8; continue;
 }
 0x61223c8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61223d1; continue;
 }
 0x61223d1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x6300020u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x61223f3; continue; }
  pc = 0x61223e8; continue;
 }
 0x61223e8 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x61223f3; continue;
 }
 0x61223f3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61223fb, "Movq");
  cpu.unimplemented(0x6122402, "Movq");
  pc = 0x6122405; continue;
 }
 0x6122405 => {
  cpu.unimplemented(0x6122405, "Movq");
  cpu.unimplemented(0x6122409, "Movq");
  cpu.unimplemented(0x612240c, "Movq");
  cpu.unimplemented(0x612240f, "Movq");
  cpu.unimplemented(0x6122412, "Psrlq");
  cpu.unimplemented(0x6122416, "Psllq");
  cpu.unimplemented(0x612241a, "Pand");
  cpu.unimplemented(0x6122421, "Por");
  cpu.unimplemented(0x6122424, "Psrlq");
  cpu.unimplemented(0x6122428, "Pand");
  cpu.unimplemented(0x612242f, "Psrlq");
  cpu.unimplemented(0x6122433, "Paddw");
  cpu.unimplemented(0x6122436, "Movq");
  cpu.unimplemented(0x6122439, "Punpcklwd");
  cpu.unimplemented(0x612243c, "Punpckhwd");
  cpu.unimplemented(0x612243f, "Movq");
  cpu.unimplemented(0x6122442, "Pand");
  cpu.unimplemented(0x6122445, "Movq");
  cpu.unimplemented(0x612244c, "Movq");
  cpu.unimplemented(0x6122450, "Pand");
  cpu.unimplemented(0x6122453, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122460, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122405; continue; }
  pc = 0x6122466; continue;
 }
 0x6122466 => {
  cpu.unimplemented(0x6122466, "Punpcklwd");
  cpu.unimplemented(0x6122469, "Punpckhwd");
  cpu.unimplemented(0x612246c, "Movq");
  cpu.unimplemented(0x612246f, "Pand");
  cpu.unimplemented(0x6122472, "Movq");
  cpu.unimplemented(0x6122479, "Movq");
  cpu.unimplemented(0x612247d, "Pand");
  cpu.unimplemented(0x6122480, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61223f3; continue; }
  pc = 0x612249a; continue;
 }
 0x612249a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61224a2, "Movq");
  cpu.unimplemented(0x61224a9, "Movq");
  pc = 0x61224ac; continue;
 }
 0x61224ac => {
  cpu.unimplemented(0x61224ac, "Movq");
  cpu.unimplemented(0x61224b0, "Movq");
  cpu.unimplemented(0x61224b3, "Movq");
  cpu.unimplemented(0x61224b6, "Movq");
  cpu.unimplemented(0x61224b9, "Psrlq");
  cpu.unimplemented(0x61224bd, "Psllq");
  cpu.unimplemented(0x61224c1, "Pand");
  cpu.unimplemented(0x61224c8, "Por");
  cpu.unimplemented(0x61224cb, "Psrlq");
  cpu.unimplemented(0x61224cf, "Pand");
  cpu.unimplemented(0x61224d6, "Psrlq");
  cpu.unimplemented(0x61224da, "Paddw");
  cpu.unimplemented(0x61224dd, "Movq");
  cpu.unimplemented(0x61224e0, "Punpcklwd");
  cpu.unimplemented(0x61224e3, "Punpckhwd");
  cpu.unimplemented(0x61224e6, "Movq");
  cpu.unimplemented(0x61224e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61224f3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61224ac; continue; }
  pc = 0x61224f9; continue;
 }
 0x61224f9 => {
  cpu.unimplemented(0x61224f9, "Punpcklwd");
  cpu.unimplemented(0x61224fc, "Punpckhwd");
  cpu.unimplemented(0x61224ff, "Movq");
  cpu.unimplemented(0x6122502, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x612251a, "Movq");
  cpu.unimplemented(0x6122521, "Movq");
  pc = 0x6122524; continue;
 }
 0x6122524 => {
  cpu.unimplemented(0x6122524, "Movq");
  cpu.unimplemented(0x6122528, "Movq");
  cpu.unimplemented(0x612252b, "Movq");
  cpu.unimplemented(0x612252e, "Movq");
  cpu.unimplemented(0x6122531, "Psrlq");
  cpu.unimplemented(0x6122535, "Psllq");
  cpu.unimplemented(0x6122539, "Pand");
  cpu.unimplemented(0x6122540, "Por");
  cpu.unimplemented(0x6122543, "Psrlq");
  cpu.unimplemented(0x6122547, "Pand");
  cpu.unimplemented(0x612254e, "Psrlq");
  cpu.unimplemented(0x6122552, "Paddw");
  cpu.unimplemented(0x6122555, "Movq");
  cpu.unimplemented(0x6122558, "Punpcklwd");
  cpu.unimplemented(0x612255b, "Punpckhwd");
  cpu.unimplemented(0x612255e, "Movq");
  cpu.unimplemented(0x6122561, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x612256b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122524; continue; }
  pc = 0x6122571; continue;
 }
 0x6122571 => {
  cpu.unimplemented(0x6122571, "Punpcklwd");
  cpu.unimplemented(0x6122574, "Punpckhwd");
  cpu.unimplemented(0x6122577, "Movq");
  cpu.unimplemented(0x612257a, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61225d9; continue; }
  pc = 0x612259b; continue;
 }
 0x612259b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61225d0; continue; }
  pc = 0x61225bf; continue;
 }
 0x61225bf => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61225d0; continue;
 }
 0x61225d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61225d9; continue;
 }
 0x61225d9 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61225e7, "Movq");
  cpu.unimplemented(0x61225ee, "Movq");
  pc = 0x61225f1; continue;
 }
 0x61225f1 => {
  cpu.unimplemented(0x61225f1, "Movq");
  cpu.unimplemented(0x61225f5, "Movq");
  cpu.unimplemented(0x61225f8, "Movq");
  cpu.unimplemented(0x61225fb, "Movq");
  cpu.unimplemented(0x61225fe, "Psrlq");
  cpu.unimplemented(0x6122602, "Psllq");
  cpu.unimplemented(0x6122606, "Pand");
  cpu.unimplemented(0x612260d, "Por");
  cpu.unimplemented(0x6122610, "Psrlq");
  cpu.unimplemented(0x6122614, "Pand");
  cpu.unimplemented(0x612261b, "Psrlq");
  cpu.unimplemented(0x612261f, "Paddw");
  cpu.unimplemented(0x6122622, "Movq");
  cpu.unimplemented(0x6122625, "Punpcklwd");
  cpu.unimplemented(0x6122628, "Punpckhwd");
  cpu.unimplemented(0x612262b, "Movq");
  cpu.unimplemented(0x612262e, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122638, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61225f1; continue; }
  pc = 0x612263e; continue;
 }
 0x612263e => {
  cpu.unimplemented(0x612263e, "Punpcklwd");
  cpu.unimplemented(0x6122641, "Punpckhwd");
  cpu.unimplemented(0x6122644, "Movq");
  cpu.unimplemented(0x6122647, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6122656; continue;
 }
 0x6122656 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x612265e, "Movq");
  cpu.unimplemented(0x6122665, "Movq");
  pc = 0x6122668; continue;
 }
 0x6122668 => {
  cpu.unimplemented(0x6122668, "Movq");
  cpu.unimplemented(0x612266c, "Movq");
  cpu.unimplemented(0x612266f, "Movq");
  cpu.unimplemented(0x6122672, "Movq");
  cpu.unimplemented(0x6122675, "Psrlq");
  cpu.unimplemented(0x6122679, "Psllq");
  cpu.unimplemented(0x612267d, "Pand");
  cpu.unimplemented(0x6122684, "Por");
  cpu.unimplemented(0x6122687, "Psrlq");
  cpu.unimplemented(0x612268b, "Pand");
  cpu.unimplemented(0x6122692, "Psrlq");
  cpu.unimplemented(0x6122696, "Paddw");
  cpu.unimplemented(0x6122699, "Movq");
  cpu.unimplemented(0x612269c, "Punpcklwd");
  cpu.unimplemented(0x612269f, "Punpckhwd");
  cpu.unimplemented(0x61226a2, "Movq");
  cpu.unimplemented(0x61226a5, "Pand");
  cpu.unimplemented(0x61226a8, "Movq");
  cpu.unimplemented(0x61226af, "Movq");
  cpu.unimplemented(0x61226b3, "Pand");
  cpu.unimplemented(0x61226b6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61226c3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122668; continue; }
  pc = 0x61226c9; continue;
 }
 0x61226c9 => {
  cpu.unimplemented(0x61226c9, "Punpcklwd");
  cpu.unimplemented(0x61226cc, "Punpckhwd");
  cpu.unimplemented(0x61226cf, "Movq");
  cpu.unimplemented(0x61226d2, "Pand");
  cpu.unimplemented(0x61226d5, "Movq");
  cpu.unimplemented(0x61226dc, "Movq");
  cpu.unimplemented(0x61226e0, "Pand");
  cpu.unimplemented(0x61226e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122656; continue; }
  pc = 0x61226fd; continue;
 }
 0x61226fd => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122705, "Movq");
  cpu.unimplemented(0x612270c, "Movq");
  pc = 0x612270f; continue;
 }
 0x612270f => {
  cpu.unimplemented(0x612270f, "Movq");
  cpu.unimplemented(0x6122713, "Movq");
  cpu.unimplemented(0x6122716, "Movq");
  cpu.unimplemented(0x6122719, "Movq");
  cpu.unimplemented(0x612271c, "Psrlq");
  cpu.unimplemented(0x6122720, "Psllq");
  cpu.unimplemented(0x6122724, "Pand");
  cpu.unimplemented(0x612272b, "Por");
  cpu.unimplemented(0x612272e, "Psrlq");
  cpu.unimplemented(0x6122732, "Pand");
  cpu.unimplemented(0x6122739, "Psrlq");
  cpu.unimplemented(0x612273d, "Paddw");
  cpu.unimplemented(0x6122740, "Movq");
  cpu.unimplemented(0x6122743, "Punpcklwd");
  cpu.unimplemented(0x6122746, "Punpckhwd");
  cpu.unimplemented(0x6122749, "Movq");
  cpu.unimplemented(0x612274c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122756, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x612270f; continue; }
  pc = 0x612275c; continue;
 }
 0x612275c => {
  cpu.unimplemented(0x612275c, "Punpcklwd");
  cpu.unimplemented(0x612275f, "Punpckhwd");
  cpu.unimplemented(0x6122762, "Movq");
  cpu.unimplemented(0x6122765, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61227c4; continue; }
  pc = 0x6122786; continue;
 }
 0x6122786 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61227bb; continue; }
  pc = 0x61227aa; continue;
 }
 0x61227aa => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61227bb; continue;
 }
 0x61227bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61227c4; continue;
 }
 0x61227c4 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61227d2, "Movq");
  cpu.unimplemented(0x61227d9, "Movq");
  pc = 0x61227dc; continue;
 }
 0x61227dc => {
  cpu.unimplemented(0x61227dc, "Movq");
  cpu.unimplemented(0x61227e0, "Movq");
  cpu.unimplemented(0x61227e3, "Movq");
  cpu.unimplemented(0x61227e6, "Movq");
  cpu.unimplemented(0x61227e9, "Psrlq");
  cpu.unimplemented(0x61227ed, "Psllq");
  cpu.unimplemented(0x61227f1, "Pand");
  cpu.unimplemented(0x61227f8, "Por");
  cpu.unimplemented(0x61227fb, "Psrlq");
  cpu.unimplemented(0x61227ff, "Pand");
  cpu.unimplemented(0x6122806, "Psrlq");
  cpu.unimplemented(0x612280a, "Paddw");
  cpu.unimplemented(0x612280d, "Movq");
  cpu.unimplemented(0x6122810, "Punpcklwd");
  cpu.unimplemented(0x6122813, "Punpckhwd");
  cpu.unimplemented(0x6122816, "Movq");
  cpu.unimplemented(0x6122819, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122823, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61227dc; continue; }
  pc = 0x6122829; continue;
 }
 0x6122829 => {
  cpu.unimplemented(0x6122829, "Punpcklwd");
  cpu.unimplemented(0x612282c, "Punpckhwd");
  cpu.unimplemented(0x612282f, "Movq");
  cpu.unimplemented(0x6122832, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x612284a, "Movq");
  cpu.unimplemented(0x6122851, "Movq");
  pc = 0x6122854; continue;
 }
 0x6122854 => {
  cpu.unimplemented(0x6122854, "Movq");
  cpu.unimplemented(0x6122858, "Movq");
  cpu.unimplemented(0x612285b, "Movq");
  cpu.unimplemented(0x612285e, "Movq");
  cpu.unimplemented(0x6122861, "Psrlq");
  cpu.unimplemented(0x6122865, "Psllq");
  cpu.unimplemented(0x6122869, "Pand");
  cpu.unimplemented(0x6122870, "Por");
  cpu.unimplemented(0x6122873, "Psrlq");
  cpu.unimplemented(0x6122877, "Pand");
  cpu.unimplemented(0x612287e, "Psrlq");
  cpu.unimplemented(0x6122882, "Paddw");
  cpu.unimplemented(0x6122885, "Movq");
  cpu.unimplemented(0x6122888, "Punpcklwd");
  cpu.unimplemented(0x612288b, "Punpckhwd");
  cpu.unimplemented(0x612288e, "Movq");
  cpu.unimplemented(0x6122891, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x612289b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122854; continue; }
  pc = 0x61228a1; continue;
 }
 0x61228a1 => {
  cpu.unimplemented(0x61228a1, "Punpcklwd");
  cpu.unimplemented(0x61228a4, "Punpckhwd");
  cpu.unimplemented(0x61228a7, "Movq");
  cpu.unimplemented(0x61228aa, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x61228b9; continue;
 }
 0x61228b9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61228c1, "Movq");
  cpu.unimplemented(0x61228c8, "Movq");
  pc = 0x61228cb; continue;
 }
 0x61228cb => {
  cpu.unimplemented(0x61228cb, "Movq");
  cpu.unimplemented(0x61228cf, "Movq");
  cpu.unimplemented(0x61228d2, "Movq");
  cpu.unimplemented(0x61228d5, "Movq");
  cpu.unimplemented(0x61228d8, "Psrlq");
  cpu.unimplemented(0x61228dc, "Psllq");
  cpu.unimplemented(0x61228e0, "Pand");
  cpu.unimplemented(0x61228e7, "Por");
  cpu.unimplemented(0x61228ea, "Psrlq");
  cpu.unimplemented(0x61228ee, "Pand");
  cpu.unimplemented(0x61228f5, "Psrlq");
  cpu.unimplemented(0x61228f9, "Paddw");
  cpu.unimplemented(0x61228fc, "Movq");
  cpu.unimplemented(0x61228ff, "Punpcklwd");
  cpu.unimplemented(0x6122902, "Punpckhwd");
  cpu.unimplemented(0x6122905, "Movq");
  cpu.unimplemented(0x6122908, "Pand");
  cpu.unimplemented(0x612290b, "Movq");
  cpu.unimplemented(0x6122912, "Movq");
  cpu.unimplemented(0x6122916, "Pand");
  cpu.unimplemented(0x6122919, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122926, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61228cb; continue; }
  pc = 0x612292c; continue;
 }
 0x612292c => {
  cpu.unimplemented(0x612292c, "Punpcklwd");
  cpu.unimplemented(0x612292f, "Punpckhwd");
  cpu.unimplemented(0x6122932, "Movq");
  cpu.unimplemented(0x6122935, "Pand");
  cpu.unimplemented(0x6122938, "Movq");
  cpu.unimplemented(0x612293f, "Movq");
  cpu.unimplemented(0x6122943, "Pand");
  cpu.unimplemented(0x6122946, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61228b9; continue; }
  pc = 0x6122960; continue;
 }
 0x6122960 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122968, "Movq");
  cpu.unimplemented(0x612296f, "Movq");
  pc = 0x6122972; continue;
 }
 0x6122972 => {
  cpu.unimplemented(0x6122972, "Movq");
  cpu.unimplemented(0x6122976, "Movq");
  cpu.unimplemented(0x6122979, "Movq");
  cpu.unimplemented(0x612297c, "Movq");
  cpu.unimplemented(0x612297f, "Psrlq");
  cpu.unimplemented(0x6122983, "Psllq");
  cpu.unimplemented(0x6122987, "Pand");
  cpu.unimplemented(0x612298e, "Por");
  cpu.unimplemented(0x6122991, "Psrlq");
  cpu.unimplemented(0x6122995, "Pand");
  cpu.unimplemented(0x612299c, "Psrlq");
  cpu.unimplemented(0x61229a0, "Paddw");
  cpu.unimplemented(0x61229a3, "Movq");
  cpu.unimplemented(0x61229a6, "Punpcklwd");
  cpu.unimplemented(0x61229a9, "Punpckhwd");
  cpu.unimplemented(0x61229ac, "Movq");
  cpu.unimplemented(0x61229af, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61229b9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122972; continue; }
  pc = 0x61229bf; continue;
 }
 0x61229bf => {
  cpu.unimplemented(0x61229bf, "Punpcklwd");
  cpu.unimplemented(0x61229c2, "Punpckhwd");
  cpu.unimplemented(0x61229c5, "Movq");
  cpu.unimplemented(0x61229c8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61229e0, "Movq");
  cpu.unimplemented(0x61229e7, "Movq");
  pc = 0x61229ea; continue;
 }
 0x61229ea => {
  cpu.unimplemented(0x61229ea, "Movq");
  cpu.unimplemented(0x61229ee, "Movq");
  cpu.unimplemented(0x61229f1, "Movq");
  cpu.unimplemented(0x61229f4, "Movq");
  cpu.unimplemented(0x61229f7, "Psrlq");
  cpu.unimplemented(0x61229fb, "Psllq");
  cpu.unimplemented(0x61229ff, "Pand");
  cpu.unimplemented(0x6122a06, "Por");
  cpu.unimplemented(0x6122a09, "Psrlq");
  cpu.unimplemented(0x6122a0d, "Pand");
  cpu.unimplemented(0x6122a14, "Psrlq");
  cpu.unimplemented(0x6122a18, "Paddw");
  cpu.unimplemented(0x6122a1b, "Movq");
  cpu.unimplemented(0x6122a1e, "Punpcklwd");
  cpu.unimplemented(0x6122a21, "Punpckhwd");
  cpu.unimplemented(0x6122a24, "Movq");
  cpu.unimplemented(0x6122a27, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122a31, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61229ea; continue; }
  pc = 0x6122a37; continue;
 }
 0x6122a37 => {
  cpu.unimplemented(0x6122a37, "Punpcklwd");
  cpu.unimplemented(0x6122a3a, "Punpckhwd");
  cpu.unimplemented(0x6122a3d, "Movq");
  cpu.unimplemented(0x6122a40, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6122a9f; continue; }
  pc = 0x6122a61; continue;
 }
 0x6122a61 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6122a96; continue; }
  pc = 0x6122a85; continue;
 }
 0x6122a85 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6122a96; continue;
 }
 0x6122a96 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6122a9f; continue;
 }
 0x6122a9f => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122aad, "Movq");
  cpu.unimplemented(0x6122ab4, "Movq");
  pc = 0x6122ab7; continue;
 }
 0x6122ab7 => {
  cpu.unimplemented(0x6122ab7, "Movq");
  cpu.unimplemented(0x6122abb, "Movq");
  cpu.unimplemented(0x6122abe, "Movq");
  cpu.unimplemented(0x6122ac1, "Movq");
  cpu.unimplemented(0x6122ac4, "Psrlq");
  cpu.unimplemented(0x6122ac8, "Psllq");
  cpu.unimplemented(0x6122acc, "Pand");
  cpu.unimplemented(0x6122ad3, "Por");
  cpu.unimplemented(0x6122ad6, "Psrlq");
  cpu.unimplemented(0x6122ada, "Pand");
  cpu.unimplemented(0x6122ae1, "Psrlq");
  cpu.unimplemented(0x6122ae5, "Paddw");
  cpu.unimplemented(0x6122ae8, "Movq");
  cpu.unimplemented(0x6122aeb, "Punpcklwd");
  cpu.unimplemented(0x6122aee, "Punpckhwd");
  cpu.unimplemented(0x6122af1, "Movq");
  cpu.unimplemented(0x6122af4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122afe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122ab7; continue; }
  pc = 0x6122b04; continue;
 }
 0x6122b04 => {
  cpu.unimplemented(0x6122b04, "Punpcklwd");
  cpu.unimplemented(0x6122b07, "Punpckhwd");
  cpu.unimplemented(0x6122b0a, "Movq");
  cpu.unimplemented(0x6122b0d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6122b1c; continue;
 }
 0x6122b1c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122b24, "Movq");
  cpu.unimplemented(0x6122b2b, "Movq");
  pc = 0x6122b2e; continue;
 }
 0x6122b2e => {
  cpu.unimplemented(0x6122b2e, "Movq");
  cpu.unimplemented(0x6122b32, "Movq");
  cpu.unimplemented(0x6122b35, "Movq");
  cpu.unimplemented(0x6122b38, "Movq");
  cpu.unimplemented(0x6122b3b, "Psrlq");
  cpu.unimplemented(0x6122b3f, "Psllq");
  cpu.unimplemented(0x6122b43, "Pand");
  cpu.unimplemented(0x6122b4a, "Por");
  cpu.unimplemented(0x6122b4d, "Psrlq");
  cpu.unimplemented(0x6122b51, "Pand");
  cpu.unimplemented(0x6122b58, "Psrlq");
  cpu.unimplemented(0x6122b5c, "Paddw");
  cpu.unimplemented(0x6122b5f, "Movq");
  cpu.unimplemented(0x6122b62, "Punpcklwd");
  cpu.unimplemented(0x6122b65, "Punpckhwd");
  cpu.unimplemented(0x6122b68, "Movq");
  cpu.unimplemented(0x6122b6b, "Pand");
  cpu.unimplemented(0x6122b6e, "Movq");
  cpu.unimplemented(0x6122b75, "Movq");
  cpu.unimplemented(0x6122b79, "Pand");
  cpu.unimplemented(0x6122b7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122b89, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122b2e; continue; }
  pc = 0x6122b8f; continue;
 }
 0x6122b8f => {
  cpu.unimplemented(0x6122b8f, "Punpcklwd");
  cpu.unimplemented(0x6122b92, "Punpckhwd");
  cpu.unimplemented(0x6122b95, "Movq");
  cpu.unimplemented(0x6122b98, "Pand");
  cpu.unimplemented(0x6122b9b, "Movq");
  cpu.unimplemented(0x6122ba2, "Movq");
  cpu.unimplemented(0x6122ba6, "Pand");
  cpu.unimplemented(0x6122ba9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122b1c; continue; }
  pc = 0x6122bc3; continue;
 }
 0x6122bc3 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122bcb, "Movq");
  cpu.unimplemented(0x6122bd2, "Movq");
  pc = 0x6122bd5; continue;
 }
 0x6122bd5 => {
  cpu.unimplemented(0x6122bd5, "Movq");
  cpu.unimplemented(0x6122bd9, "Movq");
  cpu.unimplemented(0x6122bdc, "Movq");
  cpu.unimplemented(0x6122bdf, "Movq");
  cpu.unimplemented(0x6122be2, "Psrlq");
  cpu.unimplemented(0x6122be6, "Psllq");
  cpu.unimplemented(0x6122bea, "Pand");
  cpu.unimplemented(0x6122bf1, "Por");
  cpu.unimplemented(0x6122bf4, "Psrlq");
  cpu.unimplemented(0x6122bf8, "Pand");
  cpu.unimplemented(0x6122bff, "Psrlq");
  cpu.unimplemented(0x6122c03, "Paddw");
  cpu.unimplemented(0x6122c06, "Movq");
  cpu.unimplemented(0x6122c09, "Punpcklwd");
  cpu.unimplemented(0x6122c0c, "Punpckhwd");
  cpu.unimplemented(0x6122c0f, "Movq");
  cpu.unimplemented(0x6122c12, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122c1c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122bd5; continue; }
  pc = 0x6122c22; continue;
 }
 0x6122c22 => {
  cpu.unimplemented(0x6122c22, "Punpcklwd");
  cpu.unimplemented(0x6122c25, "Punpckhwd");
  cpu.unimplemented(0x6122c28, "Movq");
  cpu.unimplemented(0x6122c2b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6122c8a; continue; }
  pc = 0x6122c4c; continue;
 }
 0x6122c4c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6122c81; continue; }
  pc = 0x6122c70; continue;
 }
 0x6122c70 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6122c81; continue;
 }
 0x6122c81 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6122c8a; continue;
 }
 0x6122c8a => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122c98, "Movq");
  cpu.unimplemented(0x6122c9f, "Movq");
  pc = 0x6122ca2; continue;
 }
 0x6122ca2 => {
  cpu.unimplemented(0x6122ca2, "Movq");
  cpu.unimplemented(0x6122ca6, "Movq");
  cpu.unimplemented(0x6122ca9, "Movq");
  cpu.unimplemented(0x6122cac, "Movq");
  cpu.unimplemented(0x6122caf, "Psrlq");
  cpu.unimplemented(0x6122cb3, "Psllq");
  cpu.unimplemented(0x6122cb7, "Pand");
  cpu.unimplemented(0x6122cbe, "Por");
  cpu.unimplemented(0x6122cc1, "Psrlq");
  cpu.unimplemented(0x6122cc5, "Pand");
  cpu.unimplemented(0x6122ccc, "Psrlq");
  cpu.unimplemented(0x6122cd0, "Paddw");
  cpu.unimplemented(0x6122cd3, "Movq");
  cpu.unimplemented(0x6122cd6, "Punpcklwd");
  cpu.unimplemented(0x6122cd9, "Punpckhwd");
  cpu.unimplemented(0x6122cdc, "Movq");
  cpu.unimplemented(0x6122cdf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122ce9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122ca2; continue; }
  pc = 0x6122cef; continue;
 }
 0x6122cef => {
  cpu.unimplemented(0x6122cef, "Punpcklwd");
  cpu.unimplemented(0x6122cf2, "Punpckhwd");
  cpu.unimplemented(0x6122cf5, "Movq");
  cpu.unimplemented(0x6122cf8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122d10, "Movq");
  cpu.unimplemented(0x6122d17, "Movq");
  pc = 0x6122d1a; continue;
 }
 0x6122d1a => {
  cpu.unimplemented(0x6122d1a, "Movq");
  cpu.unimplemented(0x6122d1e, "Movq");
  cpu.unimplemented(0x6122d21, "Movq");
  cpu.unimplemented(0x6122d24, "Movq");
  cpu.unimplemented(0x6122d27, "Psrlq");
  cpu.unimplemented(0x6122d2b, "Psllq");
  cpu.unimplemented(0x6122d2f, "Pand");
  cpu.unimplemented(0x6122d36, "Por");
  cpu.unimplemented(0x6122d39, "Psrlq");
  cpu.unimplemented(0x6122d3d, "Pand");
  cpu.unimplemented(0x6122d44, "Psrlq");
  cpu.unimplemented(0x6122d48, "Paddw");
  cpu.unimplemented(0x6122d4b, "Movq");
  cpu.unimplemented(0x6122d4e, "Punpcklwd");
  cpu.unimplemented(0x6122d51, "Punpckhwd");
  cpu.unimplemented(0x6122d54, "Movq");
  cpu.unimplemented(0x6122d57, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122d61, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122d1a; continue; }
  pc = 0x6122d67; continue;
 }
 0x6122d67 => {
  cpu.unimplemented(0x6122d67, "Punpcklwd");
  cpu.unimplemented(0x6122d6a, "Punpckhwd");
  cpu.unimplemented(0x6122d6d, "Movq");
  cpu.unimplemented(0x6122d70, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6122d7f; continue;
 }
 0x6122d7f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122d87, "Movq");
  cpu.unimplemented(0x6122d8e, "Movq");
  pc = 0x6122d91; continue;
 }
 0x6122d91 => {
  cpu.unimplemented(0x6122d91, "Movq");
  cpu.unimplemented(0x6122d95, "Movq");
  cpu.unimplemented(0x6122d98, "Movq");
  cpu.unimplemented(0x6122d9b, "Movq");
  cpu.unimplemented(0x6122d9e, "Psrlq");
  cpu.unimplemented(0x6122da2, "Psllq");
  cpu.unimplemented(0x6122da6, "Pand");
  cpu.unimplemented(0x6122dad, "Por");
  cpu.unimplemented(0x6122db0, "Psrlq");
  cpu.unimplemented(0x6122db4, "Pand");
  cpu.unimplemented(0x6122dbb, "Psrlq");
  cpu.unimplemented(0x6122dbf, "Paddw");
  cpu.unimplemented(0x6122dc2, "Movq");
  cpu.unimplemented(0x6122dc5, "Punpcklwd");
  cpu.unimplemented(0x6122dc8, "Punpckhwd");
  cpu.unimplemented(0x6122dcb, "Movq");
  cpu.unimplemented(0x6122dce, "Pand");
  cpu.unimplemented(0x6122dd1, "Movq");
  cpu.unimplemented(0x6122dd8, "Movq");
  cpu.unimplemented(0x6122ddc, "Pand");
  cpu.unimplemented(0x6122ddf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122dec, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122d91; continue; }
  pc = 0x6122df2; continue;
 }
 0x6122df2 => {
  cpu.unimplemented(0x6122df2, "Punpcklwd");
  cpu.unimplemented(0x6122df5, "Punpckhwd");
  cpu.unimplemented(0x6122df8, "Movq");
  cpu.unimplemented(0x6122dfb, "Pand");
  cpu.unimplemented(0x6122dfe, "Movq");
  cpu.unimplemented(0x6122e05, "Movq");
  cpu.unimplemented(0x6122e09, "Pand");
  cpu.unimplemented(0x6122e0c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122d7f; continue; }
  pc = 0x6122e26; continue;
 }
 0x6122e26 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6122e7b; continue; }
  pc = 0x6122e3d; continue;
 }
 0x6122e3d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6122e72; continue; }
  pc = 0x6122e61; continue;
 }
 0x6122e61 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6122e72; continue;
 }
 0x6122e72 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6122e7b; continue;
 }
 0x6122e7b => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x6122e86; continue;
 }
 0x6122e86 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122e8e, "Movq");
  cpu.unimplemented(0x6122e95, "Movq");
  pc = 0x6122e98; continue;
 }
 0x6122e98 => {
  cpu.unimplemented(0x6122e98, "Movq");
  cpu.unimplemented(0x6122e9c, "Movq");
  cpu.unimplemented(0x6122e9f, "Movq");
  cpu.unimplemented(0x6122ea2, "Movq");
  cpu.unimplemented(0x6122ea5, "Psrlq");
  cpu.unimplemented(0x6122ea9, "Psllq");
  cpu.unimplemented(0x6122ead, "Pand");
  cpu.unimplemented(0x6122eb4, "Por");
  cpu.unimplemented(0x6122eb7, "Psrlq");
  cpu.unimplemented(0x6122ebb, "Pand");
  cpu.unimplemented(0x6122ec2, "Psrlq");
  cpu.unimplemented(0x6122ec6, "Paddw");
  cpu.unimplemented(0x6122ec9, "Movq");
  cpu.unimplemented(0x6122ecc, "Punpcklwd");
  cpu.unimplemented(0x6122ecf, "Punpckhwd");
  cpu.unimplemented(0x6122ed2, "Movq");
  cpu.unimplemented(0x6122ed5, "Pand");
  cpu.unimplemented(0x6122ed8, "Movq");
  cpu.unimplemented(0x6122edf, "Movq");
  cpu.unimplemented(0x6122ee3, "Pand");
  cpu.unimplemented(0x6122ee6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122ef3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122e98; continue; }
  pc = 0x6122ef9; continue;
 }
 0x6122ef9 => {
  cpu.unimplemented(0x6122ef9, "Punpcklwd");
  cpu.unimplemented(0x6122efc, "Punpckhwd");
  cpu.unimplemented(0x6122eff, "Movq");
  cpu.unimplemented(0x6122f02, "Pand");
  cpu.unimplemented(0x6122f05, "Movq");
  cpu.unimplemented(0x6122f0c, "Movq");
  cpu.unimplemented(0x6122f10, "Pand");
  cpu.unimplemented(0x6122f13, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6122e86; continue; }
  pc = 0x6122f2d; continue;
 }
 0x6122f2d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122f35, "Movq");
  cpu.unimplemented(0x6122f3c, "Movq");
  pc = 0x6122f3f; continue;
 }
 0x6122f3f => {
  cpu.unimplemented(0x6122f3f, "Movq");
  cpu.unimplemented(0x6122f43, "Movq");
  cpu.unimplemented(0x6122f46, "Movq");
  cpu.unimplemented(0x6122f49, "Movq");
  cpu.unimplemented(0x6122f4c, "Psrlq");
  cpu.unimplemented(0x6122f50, "Psllq");
  cpu.unimplemented(0x6122f54, "Pand");
  cpu.unimplemented(0x6122f5b, "Por");
  cpu.unimplemented(0x6122f5e, "Psrlq");
  cpu.unimplemented(0x6122f62, "Pand");
  cpu.unimplemented(0x6122f69, "Psrlq");
  cpu.unimplemented(0x6122f6d, "Paddw");
  cpu.unimplemented(0x6122f70, "Movq");
  cpu.unimplemented(0x6122f73, "Punpcklwd");
  cpu.unimplemented(0x6122f76, "Punpckhwd");
  cpu.unimplemented(0x6122f79, "Movq");
  cpu.unimplemented(0x6122f7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122f86, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122f3f; continue; }
  pc = 0x6122f8c; continue;
 }
 0x6122f8c => {
  cpu.unimplemented(0x6122f8c, "Punpcklwd");
  cpu.unimplemented(0x6122f8f, "Punpckhwd");
  cpu.unimplemented(0x6122f92, "Movq");
  cpu.unimplemented(0x6122f95, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6122fad, "Movq");
  cpu.unimplemented(0x6122fb4, "Movq");
  pc = 0x6122fb7; continue;
 }
 0x6122fb7 => {
  cpu.unimplemented(0x6122fb7, "Movq");
  cpu.unimplemented(0x6122fbb, "Movq");
  cpu.unimplemented(0x6122fbe, "Movq");
  cpu.unimplemented(0x6122fc1, "Movq");
  cpu.unimplemented(0x6122fc4, "Psrlq");
  cpu.unimplemented(0x6122fc8, "Psllq");
  cpu.unimplemented(0x6122fcc, "Pand");
  cpu.unimplemented(0x6122fd3, "Por");
  cpu.unimplemented(0x6122fd6, "Psrlq");
  cpu.unimplemented(0x6122fda, "Pand");
  cpu.unimplemented(0x6122fe1, "Psrlq");
  cpu.unimplemented(0x6122fe5, "Paddw");
  cpu.unimplemented(0x6122fe8, "Movq");
  cpu.unimplemented(0x6122feb, "Punpcklwd");
  cpu.unimplemented(0x6122fee, "Punpckhwd");
  cpu.unimplemented(0x6122ff1, "Movq");
  cpu.unimplemented(0x6122ff4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6122ffe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6122fb7; continue; }
  pc = 0x6123004; continue;
 }
 0x6123004 => {
  cpu.unimplemented(0x6123004, "Punpcklwd");
  cpu.unimplemented(0x6123007, "Punpckhwd");
  cpu.unimplemented(0x612300a, "Movq");
  cpu.unimplemented(0x612300d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612306c; continue; }
  pc = 0x612302e; continue;
 }
 0x612302e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6123063; continue; }
  pc = 0x6123052; continue;
 }
 0x6123052 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6123063; continue;
 }
 0x6123063 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612306c; continue;
 }
 0x612306c => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x612307a, "Movq");
  cpu.unimplemented(0x6123081, "Movq");
  pc = 0x6123084; continue;
 }
 0x6123084 => {
  cpu.unimplemented(0x6123084, "Movq");
  cpu.unimplemented(0x6123088, "Movq");
  cpu.unimplemented(0x612308b, "Movq");
  cpu.unimplemented(0x612308e, "Movq");
  cpu.unimplemented(0x6123091, "Psrlq");
  cpu.unimplemented(0x6123095, "Psllq");
  cpu.unimplemented(0x6123099, "Pand");
  cpu.unimplemented(0x61230a0, "Por");
  cpu.unimplemented(0x61230a3, "Psrlq");
  cpu.unimplemented(0x61230a7, "Pand");
  cpu.unimplemented(0x61230ae, "Psrlq");
  cpu.unimplemented(0x61230b2, "Paddw");
  cpu.unimplemented(0x61230b5, "Movq");
  cpu.unimplemented(0x61230b8, "Punpcklwd");
  cpu.unimplemented(0x61230bb, "Punpckhwd");
  cpu.unimplemented(0x61230be, "Movq");
  cpu.unimplemented(0x61230c1, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61230cb, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6123084; continue; }
  pc = 0x61230d1; continue;
 }
 0x61230d1 => {
  cpu.unimplemented(0x61230d1, "Punpcklwd");
  cpu.unimplemented(0x61230d4, "Punpckhwd");
  cpu.unimplemented(0x61230d7, "Movq");
  cpu.unimplemented(0x61230da, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x61230e9; continue;
 }
 0x61230e9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61230f1, "Movq");
  cpu.unimplemented(0x61230f8, "Movq");
  pc = 0x61230fb; continue;
 }
 0x61230fb => {
  cpu.unimplemented(0x61230fb, "Movq");
  cpu.unimplemented(0x61230ff, "Movq");
  cpu.unimplemented(0x6123102, "Movq");
  cpu.unimplemented(0x6123105, "Movq");
  cpu.unimplemented(0x6123108, "Psrlq");
  cpu.unimplemented(0x612310c, "Psllq");
  cpu.unimplemented(0x6123110, "Pand");
  cpu.unimplemented(0x6123117, "Por");
  cpu.unimplemented(0x612311a, "Psrlq");
  cpu.unimplemented(0x612311e, "Pand");
  cpu.unimplemented(0x6123125, "Psrlq");
  cpu.unimplemented(0x6123129, "Paddw");
  cpu.unimplemented(0x612312c, "Movq");
  cpu.unimplemented(0x612312f, "Punpcklwd");
  cpu.unimplemented(0x6123132, "Punpckhwd");
  cpu.unimplemented(0x6123135, "Movq");
  cpu.unimplemented(0x6123138, "Pand");
  cpu.unimplemented(0x612313b, "Movq");
  cpu.unimplemented(0x6123142, "Movq");
  cpu.unimplemented(0x6123146, "Pand");
  cpu.unimplemented(0x6123149, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x6123156, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61230fb; continue; }
  pc = 0x612315c; continue;
 }
 0x612315c => {
  cpu.unimplemented(0x612315c, "Punpcklwd");
  cpu.unimplemented(0x612315f, "Punpckhwd");
  cpu.unimplemented(0x6123162, "Movq");
  cpu.unimplemented(0x6123165, "Pand");
  cpu.unimplemented(0x6123168, "Movq");
  cpu.unimplemented(0x612316f, "Movq");
  cpu.unimplemented(0x6123173, "Pand");
  cpu.unimplemented(0x6123176, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61230e9; continue; }
  pc = 0x6123190; continue;
 }
 0x6123190 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123198, "Movq");
  cpu.unimplemented(0x612319f, "Movq");
  pc = 0x61231a2; continue;
 }
 0x61231a2 => {
  cpu.unimplemented(0x61231a2, "Movq");
  cpu.unimplemented(0x61231a6, "Movq");
  cpu.unimplemented(0x61231a9, "Movq");
  cpu.unimplemented(0x61231ac, "Movq");
  cpu.unimplemented(0x61231af, "Psrlq");
  cpu.unimplemented(0x61231b3, "Psllq");
  cpu.unimplemented(0x61231b7, "Pand");
  cpu.unimplemented(0x61231be, "Por");
  cpu.unimplemented(0x61231c1, "Psrlq");
  cpu.unimplemented(0x61231c5, "Pand");
  cpu.unimplemented(0x61231cc, "Psrlq");
  cpu.unimplemented(0x61231d0, "Paddw");
  cpu.unimplemented(0x61231d3, "Movq");
  cpu.unimplemented(0x61231d6, "Punpcklwd");
  cpu.unimplemented(0x61231d9, "Punpckhwd");
  cpu.unimplemented(0x61231dc, "Movq");
  cpu.unimplemented(0x61231df, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61231e9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61231a2; continue; }
  pc = 0x61231ef; continue;
 }
 0x61231ef => {
  cpu.unimplemented(0x61231ef, "Punpcklwd");
  cpu.unimplemented(0x61231f2, "Punpckhwd");
  cpu.unimplemented(0x61231f5, "Movq");
  cpu.unimplemented(0x61231f8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6123257; continue; }
  pc = 0x6123219; continue;
 }
 0x6123219 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612324e; continue; }
  pc = 0x612323d; continue;
 }
 0x612323d => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612324e; continue;
 }
 0x612324e => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6123257; continue;
 }
 0x6123257 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123265, "Movq");
  cpu.unimplemented(0x612326c, "Movq");
  pc = 0x612326f; continue;
 }
 0x612326f => {
  cpu.unimplemented(0x612326f, "Movq");
  cpu.unimplemented(0x6123273, "Movq");
  cpu.unimplemented(0x6123276, "Movq");
  cpu.unimplemented(0x6123279, "Movq");
  cpu.unimplemented(0x612327c, "Psrlq");
  cpu.unimplemented(0x6123280, "Psllq");
  cpu.unimplemented(0x6123284, "Pand");
  cpu.unimplemented(0x612328b, "Por");
  cpu.unimplemented(0x612328e, "Psrlq");
  cpu.unimplemented(0x6123292, "Pand");
  cpu.unimplemented(0x6123299, "Psrlq");
  cpu.unimplemented(0x612329d, "Paddw");
  cpu.unimplemented(0x61232a0, "Movq");
  cpu.unimplemented(0x61232a3, "Punpcklwd");
  cpu.unimplemented(0x61232a6, "Punpckhwd");
  cpu.unimplemented(0x61232a9, "Movq");
  cpu.unimplemented(0x61232ac, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61232b6, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x612326f; continue; }
  pc = 0x61232bc; continue;
 }
 0x61232bc => {
  cpu.unimplemented(0x61232bc, "Punpcklwd");
  cpu.unimplemented(0x61232bf, "Punpckhwd");
  cpu.unimplemented(0x61232c2, "Movq");
  cpu.unimplemented(0x61232c5, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61232dd, "Movq");
  cpu.unimplemented(0x61232e4, "Movq");
  pc = 0x61232e7; continue;
 }
 0x61232e7 => {
  cpu.unimplemented(0x61232e7, "Movq");
  cpu.unimplemented(0x61232eb, "Movq");
  cpu.unimplemented(0x61232ee, "Movq");
  cpu.unimplemented(0x61232f1, "Movq");
  cpu.unimplemented(0x61232f4, "Psrlq");
  cpu.unimplemented(0x61232f8, "Psllq");
  cpu.unimplemented(0x61232fc, "Pand");
  cpu.unimplemented(0x6123303, "Por");
  cpu.unimplemented(0x6123306, "Psrlq");
  cpu.unimplemented(0x612330a, "Pand");
  cpu.unimplemented(0x6123311, "Psrlq");
  cpu.unimplemented(0x6123315, "Paddw");
  cpu.unimplemented(0x6123318, "Movq");
  cpu.unimplemented(0x612331b, "Punpcklwd");
  cpu.unimplemented(0x612331e, "Punpckhwd");
  cpu.unimplemented(0x6123321, "Movq");
  cpu.unimplemented(0x6123324, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x612332e, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61232e7; continue; }
  pc = 0x6123334; continue;
 }
 0x6123334 => {
  cpu.unimplemented(0x6123334, "Punpcklwd");
  cpu.unimplemented(0x6123337, "Punpckhwd");
  cpu.unimplemented(0x612333a, "Movq");
  cpu.unimplemented(0x612333d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x612366b; continue; }
  pc = 0x6123357; continue;
 }
 0x6123357 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x612335c; continue;
 }
 0x612335c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123364, "Movq");
  cpu.unimplemented(0x612336b, "Movq");
  pc = 0x612336e; continue;
 }
 0x612336e => {
  cpu.unimplemented(0x612336e, "Movq");
  cpu.unimplemented(0x6123372, "Movq");
  cpu.unimplemented(0x6123375, "Movq");
  cpu.unimplemented(0x6123378, "Movq");
  cpu.unimplemented(0x612337b, "Psrlq");
  cpu.unimplemented(0x612337f, "Psllq");
  cpu.unimplemented(0x6123383, "Pand");
  cpu.unimplemented(0x612338a, "Por");
  cpu.unimplemented(0x612338d, "Psrlq");
  cpu.unimplemented(0x6123391, "Pand");
  cpu.unimplemented(0x6123398, "Psrlq");
  cpu.unimplemented(0x612339c, "Paddw");
  cpu.unimplemented(0x612339f, "Movq");
  cpu.unimplemented(0x61233a2, "Punpcklwd");
  cpu.unimplemented(0x61233a5, "Punpckhwd");
  cpu.unimplemented(0x61233a8, "Movq");
  cpu.unimplemented(0x61233ab, "Pand");
  cpu.unimplemented(0x61233ae, "Movq");
  cpu.unimplemented(0x61233b5, "Movq");
  cpu.unimplemented(0x61233b9, "Pand");
  cpu.unimplemented(0x61233bc, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61233c9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x612336e; continue; }
  pc = 0x61233cf; continue;
 }
 0x61233cf => {
  cpu.unimplemented(0x61233cf, "Punpcklwd");
  cpu.unimplemented(0x61233d2, "Punpckhwd");
  cpu.unimplemented(0x61233d5, "Movq");
  cpu.unimplemented(0x61233d8, "Pand");
  cpu.unimplemented(0x61233db, "Movq");
  cpu.unimplemented(0x61233e2, "Movq");
  cpu.unimplemented(0x61233e6, "Pand");
  cpu.unimplemented(0x61233e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612335c; continue; }
  pc = 0x6123403; continue;
 }
 0x6123403 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x612340b, "Movq");
  cpu.unimplemented(0x6123412, "Movq");
  pc = 0x6123415; continue;
 }
 0x6123415 => {
  cpu.unimplemented(0x6123415, "Movq");
  cpu.unimplemented(0x6123419, "Movq");
  cpu.unimplemented(0x612341c, "Movq");
  cpu.unimplemented(0x612341f, "Movq");
  cpu.unimplemented(0x6123422, "Psrlq");
  cpu.unimplemented(0x6123426, "Psllq");
  cpu.unimplemented(0x612342a, "Pand");
  cpu.unimplemented(0x6123431, "Por");
  cpu.unimplemented(0x6123434, "Psrlq");
  cpu.unimplemented(0x6123438, "Pand");
  cpu.unimplemented(0x612343f, "Psrlq");
  cpu.unimplemented(0x6123443, "Paddw");
  cpu.unimplemented(0x6123446, "Movq");
  cpu.unimplemented(0x6123449, "Punpcklwd");
  cpu.unimplemented(0x612344c, "Punpckhwd");
  cpu.unimplemented(0x612344f, "Movq");
  cpu.unimplemented(0x6123452, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x612345c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6123415; continue; }
  pc = 0x6123462; continue;
 }
 0x6123462 => {
  cpu.unimplemented(0x6123462, "Punpcklwd");
  cpu.unimplemented(0x6123465, "Punpckhwd");
  cpu.unimplemented(0x6123468, "Movq");
  cpu.unimplemented(0x612346b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123483, "Movq");
  cpu.unimplemented(0x612348a, "Movq");
  pc = 0x612348d; continue;
 }
 0x612348d => {
  cpu.unimplemented(0x612348d, "Movq");
  cpu.unimplemented(0x6123491, "Movq");
  cpu.unimplemented(0x6123494, "Movq");
  cpu.unimplemented(0x6123497, "Movq");
  cpu.unimplemented(0x612349a, "Psrlq");
  cpu.unimplemented(0x612349e, "Psllq");
  cpu.unimplemented(0x61234a2, "Pand");
  cpu.unimplemented(0x61234a9, "Por");
  cpu.unimplemented(0x61234ac, "Psrlq");
  cpu.unimplemented(0x61234b0, "Pand");
  cpu.unimplemented(0x61234b7, "Psrlq");
  cpu.unimplemented(0x61234bb, "Paddw");
  cpu.unimplemented(0x61234be, "Movq");
  cpu.unimplemented(0x61234c1, "Punpcklwd");
  cpu.unimplemented(0x61234c4, "Punpckhwd");
  cpu.unimplemented(0x61234c7, "Movq");
  cpu.unimplemented(0x61234ca, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61234d4, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x612348d; continue; }
  pc = 0x61234da; continue;
 }
 0x61234da => {
  cpu.unimplemented(0x61234da, "Punpcklwd");
  cpu.unimplemented(0x61234dd, "Punpckhwd");
  cpu.unimplemented(0x61234e0, "Movq");
  cpu.unimplemented(0x61234e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6123542; continue; }
  pc = 0x6123504; continue;
 }
 0x6123504 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6123539; continue; }
  pc = 0x6123528; continue;
 }
 0x6123528 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6123539; continue;
 }
 0x6123539 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6123542; continue;
 }
 0x6123542 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123550, "Movq");
  cpu.unimplemented(0x6123557, "Movq");
  pc = 0x612355a; continue;
 }
 0x612355a => {
  cpu.unimplemented(0x612355a, "Movq");
  cpu.unimplemented(0x612355e, "Movq");
  cpu.unimplemented(0x6123561, "Movq");
  cpu.unimplemented(0x6123564, "Movq");
  cpu.unimplemented(0x6123567, "Psrlq");
  cpu.unimplemented(0x612356b, "Psllq");
  cpu.unimplemented(0x612356f, "Pand");
  cpu.unimplemented(0x6123576, "Por");
  cpu.unimplemented(0x6123579, "Psrlq");
  cpu.unimplemented(0x612357d, "Pand");
  cpu.unimplemented(0x6123584, "Psrlq");
  cpu.unimplemented(0x6123588, "Paddw");
  cpu.unimplemented(0x612358b, "Movq");
  cpu.unimplemented(0x612358e, "Punpcklwd");
  cpu.unimplemented(0x6123591, "Punpckhwd");
  cpu.unimplemented(0x6123594, "Movq");
  cpu.unimplemented(0x6123597, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61235a1, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x612355a; continue; }
  pc = 0x61235a7; continue;
 }
 0x61235a7 => {
  cpu.unimplemented(0x61235a7, "Punpcklwd");
  cpu.unimplemented(0x61235aa, "Punpckhwd");
  cpu.unimplemented(0x61235ad, "Movq");
  cpu.unimplemented(0x61235b0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x61235bf; continue;
 }
 0x61235bf => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x61235c7, "Movq");
  cpu.unimplemented(0x61235ce, "Movq");
  pc = 0x61235d1; continue;
 }
 0x61235d1 => {
  cpu.unimplemented(0x61235d1, "Movq");
  cpu.unimplemented(0x61235d5, "Movq");
  cpu.unimplemented(0x61235d8, "Movq");
  cpu.unimplemented(0x61235db, "Movq");
  cpu.unimplemented(0x61235de, "Psrlq");
  cpu.unimplemented(0x61235e2, "Psllq");
  cpu.unimplemented(0x61235e6, "Pand");
  cpu.unimplemented(0x61235ed, "Por");
  cpu.unimplemented(0x61235f0, "Psrlq");
  cpu.unimplemented(0x61235f4, "Pand");
  cpu.unimplemented(0x61235fb, "Psrlq");
  cpu.unimplemented(0x61235ff, "Paddw");
  cpu.unimplemented(0x6123602, "Movq");
  cpu.unimplemented(0x6123605, "Punpcklwd");
  cpu.unimplemented(0x6123608, "Punpckhwd");
  cpu.unimplemented(0x612360b, "Movq");
  cpu.unimplemented(0x612360e, "Pand");
  cpu.unimplemented(0x6123611, "Movq");
  cpu.unimplemented(0x6123618, "Movq");
  cpu.unimplemented(0x612361c, "Pand");
  cpu.unimplemented(0x612361f, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x612362c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x61235d1; continue; }
  pc = 0x6123632; continue;
 }
 0x6123632 => {
  cpu.unimplemented(0x6123632, "Punpcklwd");
  cpu.unimplemented(0x6123635, "Punpckhwd");
  cpu.unimplemented(0x6123638, "Movq");
  cpu.unimplemented(0x612363b, "Pand");
  cpu.unimplemented(0x612363e, "Movq");
  cpu.unimplemented(0x6123645, "Movq");
  cpu.unimplemented(0x6123649, "Pand");
  cpu.unimplemented(0x612364c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61235bf; continue; }
  pc = 0x6123666; continue;
 }
 0x6123666 => {
  pc = 0x6123717; continue;
 }
 0x612366b => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x6123670; continue;
 }
 0x6123670 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x6123678, "Movq");
  cpu.unimplemented(0x612367f, "Movq");
  pc = 0x6123682; continue;
 }
 0x6123682 => {
  cpu.unimplemented(0x6123682, "Movq");
  cpu.unimplemented(0x6123686, "Movq");
  cpu.unimplemented(0x6123689, "Movq");
  cpu.unimplemented(0x612368c, "Movq");
  cpu.unimplemented(0x612368f, "Psrlq");
  cpu.unimplemented(0x6123693, "Psllq");
  cpu.unimplemented(0x6123697, "Pand");
  cpu.unimplemented(0x612369e, "Por");
  cpu.unimplemented(0x61236a1, "Psrlq");
  cpu.unimplemented(0x61236a5, "Pand");
  cpu.unimplemented(0x61236ac, "Psrlq");
  cpu.unimplemented(0x61236b0, "Paddw");
  cpu.unimplemented(0x61236b3, "Movq");
  cpu.unimplemented(0x61236b6, "Punpcklwd");
  cpu.unimplemented(0x61236b9, "Punpckhwd");
  cpu.unimplemented(0x61236bc, "Movq");
  cpu.unimplemented(0x61236bf, "Pand");
  cpu.unimplemented(0x61236c2, "Movq");
  cpu.unimplemented(0x61236c9, "Movq");
  cpu.unimplemented(0x61236cd, "Pand");
  cpu.unimplemented(0x61236d0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x61236dd, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x6123682; continue; }
  pc = 0x61236e3; continue;
 }
 0x61236e3 => {
  cpu.unimplemented(0x61236e3, "Punpcklwd");
  cpu.unimplemented(0x61236e6, "Punpckhwd");
  cpu.unimplemented(0x61236e9, "Movq");
  cpu.unimplemented(0x61236ec, "Pand");
  cpu.unimplemented(0x61236ef, "Movq");
  cpu.unimplemented(0x61236f6, "Movq");
  cpu.unimplemented(0x61236fa, "Pand");
  cpu.unimplemented(0x61236fd, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123670; continue; }
  pc = 0x6123717; continue;
 }
 0x6123717 => {
  cpu.unimplemented(0x6123717, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612371b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612371b); }
 let mut pc: u32 = 0x612371b;
 loop { match pc {
 0x612371b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123720); }
 let mut pc: u32 = 0x6123720;
 loop { match pc {
 0x6123720 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123725); }
 let mut pc: u32 = 0x6123725;
 loop { match pc {
 0x6123725 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x625192cu32); cpu.ecx = v; }
  pc = 0x6123734; continue;
 }
 0x6123734 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x6123734; continue; }
  pc = 0x6123757; continue;
 }
 0x6123757 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123759); }
 let mut pc: u32 = 0x6123759;
 loop { match pc {
 0x6123759 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x625194cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x612379c; continue;
 }
 0x612379c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612379c; continue; }
  pc = 0x61237ae; continue;
 }
 0x61237ae => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_061237b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x61237b1); }
 let mut pc: u32 = 0x61237b1;
 loop { match pc {
 0x61237b1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x625194cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x61237f4; continue;
 }
 0x61237f4 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61237f4; continue; }
  pc = 0x6123805; continue;
 }
 0x6123805 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123808); }
 let mut pc: u32 = 0x6123808;
 loop { match pc {
 0x6123808 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x625194cu32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 1u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6123823; continue; }
  pc = 0x612381d; continue;
 }
 0x612381d => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  pc = 0x6123823; continue;
 }
 0x6123823 => {
  { let v: u32 = cpu.r32(0x6251948u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x612382b; continue;
 }
 0x612382b => {
  { let v: u32 = cpu.r32(0x625194cu32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x6123852; continue;
 }
 0x6123836 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x6123852; continue;
 }
 0x6123852 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123836; continue; }
  pc = 0x6123855; continue;
 }
 0x6123855 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612382b; continue; }
  pc = 0x612385a; continue;
 }
 0x612385a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612385c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612385c); }
 let mut pc: u32 = 0x612385c;
 loop { match pc {
 0x612385c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x612387b; continue; }
  pc = 0x6123878; continue;
 }
 0x6123878 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x612387b; continue;
 }
 0x612387b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x625194cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x61238a9; continue;
 }
 0x61238a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x61238bd; continue;
 }
 0x61238ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x61238bd; continue;
 }
 0x61238bd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61238ae; continue; }
  pc = 0x61238c0; continue;
 }
 0x61238c0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61238a9; continue; }
  pc = 0x61238c9; continue;
 }
 0x61238c9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_061238cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x61238cc); }
 let mut pc: u32 = 0x61238cc;
 loop { match pc {
 0x61238cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x61238eb; continue; }
  pc = 0x61238e8; continue;
 }
 0x61238e8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x61238eb; continue;
 }
 0x61238eb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x625194cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x6123919; continue;
 }
 0x6123919 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x6123921; continue;
 }
 0x612391e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x6123921; continue;
 }
 0x6123921 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612391e; continue; }
  pc = 0x6123924; continue;
 }
 0x6123924 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123919; continue; }
  pc = 0x612392d; continue;
 }
 0x612392d => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123930); }
 let mut pc: u32 = 0x6123930;
 loop { match pc {
 0x6123930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300034u32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x630001cu32, v); }
  pc = 0x612394d; continue;
 }
 0x612394d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123960; continue; }
  pc = 0x612395a; continue;
 }
 0x612395a => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123960; continue;
 }
 0x6123960 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x61239a4; continue;
 }
 0x6123973 => {
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
  pc = 0x61239a4; continue;
 }
 0x61239a4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123973; continue; }
  pc = 0x61239ac; continue;
 }
 0x61239ac => {
  { let v: u32 = cpu.dec32(cpu.r32(0x630001cu32)); cpu.w32(0x630001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612394d; continue; }
  pc = 0x61239b4; continue;
 }
 0x61239b4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_061239b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x61239b6); }
 let mut pc: u32 = 0x61239b6;
 loop { match pc {
 0x61239b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x500u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300034u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300024u32, v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300028u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x630002cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x6300030u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x630001cu32, v); }
  pc = 0x6123a26; continue;
 }
 0x6123a26 => {
  { let v: u32 = cpu.r32(0x630002cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123a3a; continue; }
  pc = 0x6123a34; continue;
 }
 0x6123a34 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123a3a; continue;
 }
 0x6123a3a => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6123a7e; continue;
 }
 0x6123a4d => {
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
  pc = 0x6123a7e; continue;
 }
 0x6123a7e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123a4d; continue; }
  pc = 0x6123a86; continue;
 }
 0x6123a86 => {
  { let v: u32 = cpu.r32(0x6300030u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123a9a; continue; }
  pc = 0x6123a94; continue;
 }
 0x6123a94 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123a9a; continue;
 }
 0x6123a9a => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebx = v; }
  pc = 0x6123ab9; continue;
 }
 0x6123aa2 => {
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
  pc = 0x6123ab9; continue;
 }
 0x6123ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123aa2; continue; }
  pc = 0x6123abc; continue;
 }
 0x6123abc => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x6300024u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x6300028u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x6300028u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x630001cu32)); cpu.w32(0x630001cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123a26; continue; }
  pc = 0x6123ada; continue;
 }
 0x6123ada => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123ade); }
 let mut pc: u32 = 0x6123ade;
 loop { match pc {
 0x6123ade => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x6123b15; continue;
 }
 0x6123b15 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x6123b2c; continue;
 }
 0x6123b22 => {
  { cpu.lods(1, false); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x6123b2c; continue;
 }
 0x6123b2c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123b22; continue; }
  pc = 0x6123b2f; continue;
 }
 0x6123b2f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123b15; continue; }
  pc = 0x6123b3a; continue;
 }
 0x6123b3a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06123b3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6123b3d); }
 let mut pc: u32 = 0x6123b3d;
 loop { match pc {
 0x6123b3d => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6123b8f; continue; }
  pc = 0x6123b51; continue;
 }
 0x6123b51 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6123b86; continue; }
  pc = 0x6123b75; continue;
 }
 0x6123b75 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6123b86; continue;
 }
 0x6123b86 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6123b8f; continue;
 }
 0x6123b8f => {
  { let v: u32 = cpu.edi; cpu.w32(0x6300020u32, v); }
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300034u32, v); }
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6123bc9; continue; }
  pc = 0x6123bb8; continue;
 }
 0x6123bb8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x6400u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6400u32); cpu.edx = v; }
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x6123bc9; continue;
 }
 0x6123bc9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123bdd; continue; }
  pc = 0x6123bd7; continue;
 }
 0x6123bd7 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123bdd; continue;
 }
 0x6123bdd => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6123c7c; continue;
 }
 0x6123bf3 => {
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
  pc = 0x6123c7c; continue;
 }
 0x6123c7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123bf3; continue; }
  pc = 0x6123c88; continue;
 }
 0x6123c88 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123bc9; continue; }
  pc = 0x6123c9c; continue;
 }
 0x6123c9c => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123cb0; continue; }
  pc = 0x6123caa; continue;
 }
 0x6123caa => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123cb0; continue;
 }
 0x6123cb0 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6123d15; continue;
 }
 0x6123cc8 => {
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
  pc = 0x6123d15; continue;
 }
 0x6123d15 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123cc8; continue; }
  pc = 0x6123d1d; continue;
 }
 0x6123d1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6123d74; continue; }
  pc = 0x6123d36; continue;
 }
 0x6123d36 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6123d6b; continue; }
  pc = 0x6123d5a; continue;
 }
 0x6123d5a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6123d6b; continue;
 }
 0x6123d6b => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6123d74; continue;
 }
 0x6123d74 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123d8f; continue; }
  pc = 0x6123d89; continue;
 }
 0x6123d89 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123d8f; continue;
 }
 0x6123d8f => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6123df4; continue;
 }
 0x6123da7 => {
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
  pc = 0x6123df4; continue;
 }
 0x6123df4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123da7; continue; }
  pc = 0x6123dfc; continue;
 }
 0x6123dfc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123e17; continue; }
  pc = 0x6123e11; continue;
 }
 0x6123e11 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123e17; continue;
 }
 0x6123e17 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6123e7c; continue;
 }
 0x6123e2f => {
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
  pc = 0x6123e7c; continue;
 }
 0x6123e7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123e2f; continue; }
  pc = 0x6123e84; continue;
 }
 0x6123e84 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x6123e8a; continue;
 }
 0x6123e8a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123e9e; continue; }
  pc = 0x6123e98; continue;
 }
 0x6123e98 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123e9e; continue;
 }
 0x6123e9e => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6123f3d; continue;
 }
 0x6123eb4 => {
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
  pc = 0x6123f3d; continue;
 }
 0x6123f3d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123eb4; continue; }
  pc = 0x6123f49; continue;
 }
 0x6123f49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6123e8a; continue; }
  pc = 0x6123f5d; continue;
 }
 0x6123f5d => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6123f71; continue; }
  pc = 0x6123f6b; continue;
 }
 0x6123f6b => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6123f71; continue;
 }
 0x6123f71 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6123fd6; continue;
 }
 0x6123f89 => {
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
  pc = 0x6123fd6; continue;
 }
 0x6123fd6 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6123f89; continue; }
  pc = 0x6123fde; continue;
 }
 0x6123fde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6124035; continue; }
  pc = 0x6123ff7; continue;
 }
 0x6123ff7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612402c; continue; }
  pc = 0x612401b; continue;
 }
 0x612401b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612402c; continue;
 }
 0x612402c => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6124035; continue;
 }
 0x6124035 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124050; continue; }
  pc = 0x612404a; continue;
 }
 0x612404a => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124050; continue;
 }
 0x6124050 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x61240b5; continue;
 }
 0x6124068 => {
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
  pc = 0x61240b5; continue;
 }
 0x61240b5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124068; continue; }
  pc = 0x61240bd; continue;
 }
 0x61240bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61240d8; continue; }
  pc = 0x61240d2; continue;
 }
 0x61240d2 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61240d8; continue;
 }
 0x61240d8 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x612413d; continue;
 }
 0x61240f0 => {
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
  pc = 0x612413d; continue;
 }
 0x612413d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61240f0; continue; }
  pc = 0x6124145; continue;
 }
 0x6124145 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x612414b; continue;
 }
 0x612414b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x612415f; continue; }
  pc = 0x6124159; continue;
 }
 0x6124159 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x612415f; continue;
 }
 0x612415f => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x61241fe; continue;
 }
 0x6124175 => {
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
  pc = 0x61241fe; continue;
 }
 0x61241fe => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124175; continue; }
  pc = 0x612420a; continue;
 }
 0x612420a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612414b; continue; }
  pc = 0x612421e; continue;
 }
 0x612421e => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124232; continue; }
  pc = 0x612422c; continue;
 }
 0x612422c => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124232; continue;
 }
 0x6124232 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6124297; continue;
 }
 0x612424a => {
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
  pc = 0x6124297; continue;
 }
 0x6124297 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x612424a; continue; }
  pc = 0x612429f; continue;
 }
 0x612429f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61242ba; continue; }
  pc = 0x61242b4; continue;
 }
 0x61242b4 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61242ba; continue;
 }
 0x61242ba => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x612431f; continue;
 }
 0x61242d2 => {
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
  pc = 0x612431f; continue;
 }
 0x612431f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61242d2; continue; }
  pc = 0x6124327; continue;
 }
 0x6124327 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612437e; continue; }
  pc = 0x6124340; continue;
 }
 0x6124340 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124375; continue; }
  pc = 0x6124364; continue;
 }
 0x6124364 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124375; continue;
 }
 0x6124375 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612437e; continue;
 }
 0x612437e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x61247b5; continue; }
  pc = 0x612438f; continue;
 }
 0x612438f => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61243a9; continue; }
  pc = 0x61243a3; continue;
 }
 0x61243a3 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61243a9; continue;
 }
 0x61243a9 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x612440e; continue;
 }
 0x61243c1 => {
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
  pc = 0x612440e; continue;
 }
 0x612440e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61243c1; continue; }
  pc = 0x6124416; continue;
 }
 0x6124416 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x612441c; continue;
 }
 0x612441c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124430; continue; }
  pc = 0x612442a; continue;
 }
 0x612442a => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124430; continue;
 }
 0x6124430 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x61244cf; continue;
 }
 0x6124446 => {
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
  pc = 0x61244cf; continue;
 }
 0x61244cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124446; continue; }
  pc = 0x61244db; continue;
 }
 0x61244db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612441c; continue; }
  pc = 0x61244ef; continue;
 }
 0x61244ef => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124503; continue; }
  pc = 0x61244fd; continue;
 }
 0x61244fd => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124503; continue;
 }
 0x6124503 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6124568; continue;
 }
 0x612451b => {
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
  pc = 0x6124568; continue;
 }
 0x6124568 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x612451b; continue; }
  pc = 0x6124570; continue;
 }
 0x6124570 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x612458b; continue; }
  pc = 0x6124585; continue;
 }
 0x6124585 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x612458b; continue;
 }
 0x612458b => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x61245f0; continue;
 }
 0x61245a3 => {
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
  pc = 0x61245f0; continue;
 }
 0x61245f0 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61245a3; continue; }
  pc = 0x61245f8; continue;
 }
 0x61245f8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612464f; continue; }
  pc = 0x6124611; continue;
 }
 0x6124611 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124646; continue; }
  pc = 0x6124635; continue;
 }
 0x6124635 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124646; continue;
 }
 0x6124646 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612464f; continue;
 }
 0x612464f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x612466a; continue; }
  pc = 0x6124664; continue;
 }
 0x6124664 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x612466a; continue;
 }
 0x612466a => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x61246cf; continue;
 }
 0x6124682 => {
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
  pc = 0x61246cf; continue;
 }
 0x61246cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124682; continue; }
  pc = 0x61246d7; continue;
 }
 0x61246d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x61246dd; continue;
 }
 0x61246dd => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61246f1; continue; }
  pc = 0x61246eb; continue;
 }
 0x61246eb => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61246f1; continue;
 }
 0x61246f1 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124790; continue;
 }
 0x6124707 => {
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
  pc = 0x6124790; continue;
 }
 0x6124790 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124707; continue; }
  pc = 0x612479c; continue;
 }
 0x612479c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61246dd; continue; }
  pc = 0x61247b0; continue;
 }
 0x61247b0 => {
  pc = 0x6124915; continue;
 }
 0x61247b5 => {
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61247cf; continue; }
  pc = 0x61247c9; continue;
 }
 0x61247c9 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61247cf; continue;
 }
 0x61247cf => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x6124834; continue;
 }
 0x61247e7 => {
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
  pc = 0x6124834; continue;
 }
 0x6124834 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61247e7; continue; }
  pc = 0x612483c; continue;
 }
 0x612483c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2eu32; cpu.ebp = v; }
  pc = 0x6124842; continue;
 }
 0x6124842 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124856; continue; }
  pc = 0x6124850; continue;
 }
 0x6124850 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124856; continue;
 }
 0x6124856 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x61248f5; continue;
 }
 0x612486c => {
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
  pc = 0x61248f5; continue;
 }
 0x61248f5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x612486c; continue; }
  pc = 0x6124901; continue;
 }
 0x6124901 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6124842; continue; }
  pc = 0x6124915; continue;
 }
 0x6124915 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06124918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6124918); }
 let mut pc: u32 = 0x6124918;
 loop { match pc {
 0x6124918 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x6250ee0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300034u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6124947; continue; }
  pc = 0x6124944; continue;
 }
 0x6124944 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x6124947; continue;
 }
 0x6124947 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61249b6; continue; }
  pc = 0x6124978; continue;
 }
 0x6124978 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61249ad; continue; }
  pc = 0x612499c; continue;
 }
 0x612499c => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61249ad; continue;
 }
 0x61249ad => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61249b6; continue;
 }
 0x61249b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x61249ee; continue;
 }
 0x61249ee => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x6124a85; continue; }
  pc = 0x61249fa; continue;
 }
 0x61249fa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124a0d; continue; }
  pc = 0x6124a07; continue;
 }
 0x6124a07 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124a0d; continue;
 }
 0x6124a0d => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124a64; continue;
 }
 0x6124a1f => {
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
  pc = 0x6124a64; continue;
 }
 0x6124a64 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124a1f; continue; }
  pc = 0x6124a6c; continue;
 }
 0x6124a6c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61249ee; continue; }
  pc = 0x6124a80; continue;
 }
 0x6124a80 => {
  pc = 0x6125004; continue;
 }
 0x6124a85 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x6124c2b; continue; }
  pc = 0x6124a96; continue;
 }
 0x6124a96 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124aa8; continue; }
  pc = 0x6124aa2; continue;
 }
 0x6124aa2 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124aa8; continue;
 }
 0x6124aa8 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124ae2; continue;
 }
 0x6124aba => {
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
  pc = 0x6124ae2; continue;
 }
 0x6124ae2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124aba; continue; }
  pc = 0x6124aea; continue;
 }
 0x6124aea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6124b43; continue; }
  pc = 0x6124b05; continue;
 }
 0x6124b05 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124b3a; continue; }
  pc = 0x6124b29; continue;
 }
 0x6124b29 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124b3a; continue;
 }
 0x6124b3a => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6124b43; continue;
 }
 0x6124b43 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124b67; continue; }
  pc = 0x6124b61; continue;
 }
 0x6124b61 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124b67; continue;
 }
 0x6124b67 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124ba1; continue;
 }
 0x6124b79 => {
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
  pc = 0x6124ba1; continue;
 }
 0x6124ba1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124b79; continue; }
  pc = 0x6124ba9; continue;
 }
 0x6124ba9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124bca; continue; }
  pc = 0x6124bc4; continue;
 }
 0x6124bc4 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124bca; continue;
 }
 0x6124bca => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124c04; continue;
 }
 0x6124bdc => {
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
  pc = 0x6124c04; continue;
 }
 0x6124c04 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124bdc; continue; }
  pc = 0x6124c0c; continue;
 }
 0x6124c0c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61249ee; continue; }
  pc = 0x6124c26; continue;
 }
 0x6124c26 => {
  pc = 0x6125004; continue;
 }
 0x6124c2b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x6124ed1; continue; }
  pc = 0x6124c39; continue;
 }
 0x6124c39 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x6124ddf; continue; }
  pc = 0x6124c42; continue;
 }
 0x6124c42 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124c55; continue; }
  pc = 0x6124c4f; continue;
 }
 0x6124c4f => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124c55; continue;
 }
 0x6124c55 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124c8f; continue;
 }
 0x6124c67 => {
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
  pc = 0x6124c8f; continue;
 }
 0x6124c8f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124c67; continue; }
  pc = 0x6124c97; continue;
 }
 0x6124c97 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124cbf; continue; }
  pc = 0x6124cb9; continue;
 }
 0x6124cb9 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124cbf; continue;
 }
 0x6124cbf => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124cf9; continue;
 }
 0x6124cd1 => {
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
  pc = 0x6124cf9; continue;
 }
 0x6124cf9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124cd1; continue; }
  pc = 0x6124d01; continue;
 }
 0x6124d01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6124d5a; continue; }
  pc = 0x6124d1c; continue;
 }
 0x6124d1c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124d51; continue; }
  pc = 0x6124d40; continue;
 }
 0x6124d40 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124d51; continue;
 }
 0x6124d51 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6124d5a; continue;
 }
 0x6124d5a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124d7e; continue; }
  pc = 0x6124d78; continue;
 }
 0x6124d78 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124d7e; continue;
 }
 0x6124d7e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124db8; continue;
 }
 0x6124d90 => {
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
  pc = 0x6124db8; continue;
 }
 0x6124db8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124d90; continue; }
  pc = 0x6124dc0; continue;
 }
 0x6124dc0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61249ee; continue; }
  pc = 0x6124dda; continue;
 }
 0x6124dda => {
  pc = 0x6125004; continue;
 }
 0x6124ddf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124df2; continue; }
  pc = 0x6124dec; continue;
 }
 0x6124dec => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124df2; continue;
 }
 0x6124df2 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124e49; continue;
 }
 0x6124e04 => {
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
  pc = 0x6124e49; continue;
 }
 0x6124e49 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124e04; continue; }
  pc = 0x6124e51; continue;
 }
 0x6124e51 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6124eaa; continue; }
  pc = 0x6124e6c; continue;
 }
 0x6124e6c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124ea1; continue; }
  pc = 0x6124e90; continue;
 }
 0x6124e90 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124ea1; continue;
 }
 0x6124ea1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6124eaa; continue;
 }
 0x6124eaa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x6251904u32)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61249ee; continue; }
  pc = 0x6124ecc; continue;
 }
 0x6124ecc => {
  pc = 0x6125004; continue;
 }
 0x6124ed1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124ee4; continue; }
  pc = 0x6124ede; continue;
 }
 0x6124ede => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124ee4; continue;
 }
 0x6124ee4 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124f1e; continue;
 }
 0x6124ef6 => {
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
  pc = 0x6124f1e; continue;
 }
 0x6124f1e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124ef6; continue; }
  pc = 0x6124f26; continue;
 }
 0x6124f26 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6124f85; continue; }
  pc = 0x6124f47; continue;
 }
 0x6124f47 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6124f7c; continue; }
  pc = 0x6124f6b; continue;
 }
 0x6124f6b => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6124f7c; continue;
 }
 0x6124f7c => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6124f85; continue;
 }
 0x6124f85 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6124fae; continue; }
  pc = 0x6124fa8; continue;
 }
 0x6124fa8 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6124fae; continue;
 }
 0x6124fae => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x6300018u32, v); }
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  pc = 0x6124fe8; continue;
 }
 0x6124fc0 => {
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
  pc = 0x6124fe8; continue;
 }
 0x6124fe8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6124fc0; continue; }
  pc = 0x6124ff0; continue;
 }
 0x6124ff0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61249ee; continue; }
  pc = 0x6125004; continue;
 }
 0x6125004 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125007); }
 let mut pc: u32 = 0x6125007;
 loop { match pc {
 0x6125007 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x6251900u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x6125026; continue; }
  pc = 0x6125023; continue;
 }
 0x6125023 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x6125026; continue;
 }
 0x6125026 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x625194cu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x6251928u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x6251940u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612508e; continue; }
  pc = 0x6125050; continue;
 }
 0x6125050 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125085; continue; }
  pc = 0x6125074; continue;
 }
 0x6125074 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125085; continue;
 }
 0x6125085 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612508e; continue;
 }
 0x612508e => {
  { let v: u32 = cpu.r32(0x6251928u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x6251904u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x61250c8; continue;
 }
 0x61250c8 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x6251944u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x61250fb; continue; }
  pc = 0x61250d0; continue;
 }
 0x61250d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x61250e9; continue;
 }
 0x61250de => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x61250e9; continue;
 }
 0x61250e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250de; continue; }
  pc = 0x61250ec; continue;
 }
 0x61250ec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250c8; continue; }
  pc = 0x61250f6; continue;
 }
 0x61250f6 => {
  pc = 0x6125427; continue;
 }
 0x61250fb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251944u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x61251e8; continue; }
  pc = 0x612510c; continue;
 }
 0x612510c => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x6125124; continue;
 }
 0x6125119 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x6125124; continue;
 }
 0x6125124 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125119; continue; }
  pc = 0x6125127; continue;
 }
 0x6125127 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612517f; continue; }
  pc = 0x6125141; continue;
 }
 0x6125141 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125176; continue; }
  pc = 0x6125165; continue;
 }
 0x6125165 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125176; continue;
 }
 0x6125176 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612517f; continue;
 }
 0x612517f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251944u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x61251a9; continue;
 }
 0x612519e => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x61251a9; continue;
 }
 0x61251a9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612519e; continue; }
  pc = 0x61251ac; continue;
 }
 0x61251ac => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x61251cc; continue;
 }
 0x61251c1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x61251cc; continue;
 }
 0x61251cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61251c1; continue; }
  pc = 0x61251cf; continue;
 }
 0x61251cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250c8; continue; }
  pc = 0x61251e3; continue;
 }
 0x61251e3 => {
  pc = 0x6125427; continue;
 }
 0x61251e8 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x6251904u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x6125373; continue; }
  pc = 0x61251f6; continue;
 }
 0x61251f6 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x61252e3; continue; }
  pc = 0x61251ff; continue;
 }
 0x61251ff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x6125218; continue;
 }
 0x612520d => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x6125218; continue;
 }
 0x6125218 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612520d; continue; }
  pc = 0x612521b; continue;
 }
 0x612521b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x6251904u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x6125242; continue;
 }
 0x6125237 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x6125242; continue;
 }
 0x6125242 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125237; continue; }
  pc = 0x6125245; continue;
 }
 0x6125245 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612529d; continue; }
  pc = 0x612525f; continue;
 }
 0x612525f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125294; continue; }
  pc = 0x6125283; continue;
 }
 0x6125283 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125294; continue;
 }
 0x6125294 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612529d; continue;
 }
 0x612529d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x6251904u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x61252c7; continue;
 }
 0x61252bc => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x61252c7; continue;
 }
 0x61252c7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61252bc; continue; }
  pc = 0x61252ca; continue;
 }
 0x61252ca => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250c8; continue; }
  pc = 0x61252de; continue;
 }
 0x61252de => {
  pc = 0x6125427; continue;
 }
 0x61252e3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x61252fc; continue;
 }
 0x61252f1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x61252fc; continue;
 }
 0x61252fc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x61252f1; continue; }
  pc = 0x61252ff; continue;
 }
 0x61252ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6125357; continue; }
  pc = 0x6125319; continue;
 }
 0x6125319 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x612534e; continue; }
  pc = 0x612533d; continue;
 }
 0x612533d => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x612534e; continue;
 }
 0x612534e => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6125357; continue;
 }
 0x6125357 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250c8; continue; }
  pc = 0x612536e; continue;
 }
 0x612536e => {
  pc = 0x6125427; continue;
 }
 0x6125373 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x612538c; continue;
 }
 0x6125381 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x612538c; continue;
 }
 0x612538c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125381; continue; }
  pc = 0x612538f; continue;
 }
 0x612538f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61253ea; continue; }
  pc = 0x61253ac; continue;
 }
 0x61253ac => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61253e1; continue; }
  pc = 0x61253d0; continue;
 }
 0x61253d0 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61253e1; continue;
 }
 0x61253e1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61253ea; continue;
 }
 0x61253ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x6251904u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x6251928u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x6251960u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x6125416; continue;
 }
 0x612540b => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x6125416; continue;
 }
 0x6125416 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x612540b; continue; }
  pc = 0x6125419; continue;
 }
 0x6125419 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x61250c8; continue; }
  pc = 0x6125427; continue;
 }
 0x6125427 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0612542a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x612542a); }
 let mut pc: u32 = 0x612542a;
 loop { match pc {
 0x612542a => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612547c; continue; }
  pc = 0x612543e; continue;
 }
 0x612543e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125473; continue; }
  pc = 0x6125462; continue;
 }
 0x6125462 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125473; continue;
 }
 0x6125473 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612547c; continue;
 }
 0x612547c => {
  { let v: u32 = cpu.edi; cpu.w32(0x6300020u32, v); }
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300034u32, v); }
  { let v: u32 = 0x66u32; cpu.ebp = v; }
  pc = 0x6125499; continue;
 }
 0x6125499 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61254ad; continue; }
  pc = 0x61254a7; continue;
 }
 0x61254a7 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61254ad; continue;
 }
 0x61254ad => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x61254f1; continue;
 }
 0x61254c0 => {
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
  pc = 0x61254f1; continue;
 }
 0x61254f1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61254c0; continue; }
  pc = 0x61254f9; continue;
 }
 0x61254f9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125499; continue; }
  pc = 0x61254fd; continue;
 }
 0x61254fd => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125511; continue; }
  pc = 0x612550b; continue;
 }
 0x612550b => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125511; continue;
 }
 0x6125511 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125555; continue;
 }
 0x6125524 => {
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
  pc = 0x6125555; continue;
 }
 0x6125555 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125524; continue; }
  pc = 0x612555d; continue;
 }
 0x612555d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61255b4; continue; }
  pc = 0x6125576; continue;
 }
 0x6125576 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61255ab; continue; }
  pc = 0x612559a; continue;
 }
 0x612559a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61255ab; continue;
 }
 0x61255ab => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61255b4; continue;
 }
 0x61255b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61255cf; continue; }
  pc = 0x61255c9; continue;
 }
 0x61255c9 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61255cf; continue;
 }
 0x61255cf => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125613; continue;
 }
 0x61255e2 => {
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
  pc = 0x6125613; continue;
 }
 0x6125613 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61255e2; continue; }
  pc = 0x612561b; continue;
 }
 0x612561b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125630; continue; }
  pc = 0x612562a; continue;
 }
 0x612562a => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125630; continue;
 }
 0x6125630 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125674; continue;
 }
 0x6125643 => {
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
  pc = 0x6125674; continue;
 }
 0x6125674 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125643; continue; }
  pc = 0x612567c; continue;
 }
 0x612567c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x6125682; continue;
 }
 0x6125682 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125696; continue; }
  pc = 0x6125690; continue;
 }
 0x6125690 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125696; continue;
 }
 0x6125696 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x61256da; continue;
 }
 0x61256a9 => {
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
  pc = 0x61256da; continue;
 }
 0x61256da => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61256a9; continue; }
  pc = 0x61256e2; continue;
 }
 0x61256e2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125682; continue; }
  pc = 0x61256e6; continue;
 }
 0x61256e6 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61256fa; continue; }
  pc = 0x61256f4; continue;
 }
 0x61256f4 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61256fa; continue;
 }
 0x61256fa => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x612573e; continue;
 }
 0x612570d => {
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
  pc = 0x612573e; continue;
 }
 0x612573e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x612570d; continue; }
  pc = 0x6125746; continue;
 }
 0x6125746 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x612579d; continue; }
  pc = 0x612575f; continue;
 }
 0x612575f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125794; continue; }
  pc = 0x6125783; continue;
 }
 0x6125783 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125794; continue;
 }
 0x6125794 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x612579d; continue;
 }
 0x612579d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61257b8; continue; }
  pc = 0x61257b2; continue;
 }
 0x61257b2 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61257b8; continue;
 }
 0x61257b8 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x61257fc; continue;
 }
 0x61257cb => {
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
  pc = 0x61257fc; continue;
 }
 0x61257fc => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61257cb; continue; }
  pc = 0x6125804; continue;
 }
 0x6125804 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125819; continue; }
  pc = 0x6125813; continue;
 }
 0x6125813 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125819; continue;
 }
 0x6125819 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x612585d; continue;
 }
 0x612582c => {
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
  pc = 0x612585d; continue;
 }
 0x612585d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x612582c; continue; }
  pc = 0x6125865; continue;
 }
 0x6125865 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x612586b; continue;
 }
 0x612586b => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x612587f; continue; }
  pc = 0x6125879; continue;
 }
 0x6125879 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x612587f; continue;
 }
 0x612587f => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x61258c3; continue;
 }
 0x6125892 => {
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
  pc = 0x61258c3; continue;
 }
 0x61258c3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125892; continue; }
  pc = 0x61258cb; continue;
 }
 0x61258cb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x612586b; continue; }
  pc = 0x61258cf; continue;
 }
 0x61258cf => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x61258e3; continue; }
  pc = 0x61258dd; continue;
 }
 0x61258dd => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x61258e3; continue;
 }
 0x61258e3 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125927; continue;
 }
 0x61258f6 => {
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
  pc = 0x6125927; continue;
 }
 0x6125927 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x61258f6; continue; }
  pc = 0x612592f; continue;
 }
 0x612592f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125944; continue; }
  pc = 0x612593e; continue;
 }
 0x612593e => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125944; continue;
 }
 0x6125944 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125988; continue;
 }
 0x6125957 => {
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
  pc = 0x6125988; continue;
 }
 0x6125988 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125957; continue; }
  pc = 0x6125990; continue;
 }
 0x6125990 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x61259e7; continue; }
  pc = 0x61259a9; continue;
 }
 0x61259a9 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x61259de; continue; }
  pc = 0x61259cd; continue;
 }
 0x61259cd => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x61259de; continue;
 }
 0x61259de => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x61259e7; continue;
 }
 0x61259e7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125a02; continue; }
  pc = 0x61259fc; continue;
 }
 0x61259fc => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125a02; continue;
 }
 0x6125a02 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125a46; continue;
 }
 0x6125a15 => {
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
  pc = 0x6125a46; continue;
 }
 0x6125a46 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125a15; continue; }
  pc = 0x6125a4e; continue;
 }
 0x6125a4e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x6125a54; continue;
 }
 0x6125a54 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125a68; continue; }
  pc = 0x6125a62; continue;
 }
 0x6125a62 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125a68; continue;
 }
 0x6125a68 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125aac; continue;
 }
 0x6125a7b => {
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
  pc = 0x6125aac; continue;
 }
 0x6125aac => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125a7b; continue; }
  pc = 0x6125ab4; continue;
 }
 0x6125ab4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125a54; continue; }
  pc = 0x6125ab8; continue;
 }
 0x6125ab8 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125acc; continue; }
  pc = 0x6125ac6; continue;
 }
 0x6125ac6 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125acc; continue;
 }
 0x6125acc => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125b10; continue;
 }
 0x6125adf => {
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
  pc = 0x6125b10; continue;
 }
 0x6125b10 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125adf; continue; }
  pc = 0x6125b18; continue;
 }
 0x6125b18 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125b2d; continue; }
  pc = 0x6125b27; continue;
 }
 0x6125b27 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125b2d; continue;
 }
 0x6125b2d => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125b71; continue;
 }
 0x6125b40 => {
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
  pc = 0x6125b71; continue;
 }
 0x6125b71 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125b40; continue; }
  pc = 0x6125b79; continue;
 }
 0x6125b79 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x6251940u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x625199au32)); }
  if cpu.zf { pc = 0x6125bd0; continue; }
  pc = 0x6125b92; continue;
 }
 0x6125b92 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x6250ec4u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125bc7; continue; }
  pc = 0x6125bb6; continue;
 }
 0x6125bb6 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125bc7; continue;
 }
 0x6125bc7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x6250ec4u32, v); }
  pc = 0x6125bd0; continue;
 }
 0x6125bd0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x6300020u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125beb; continue; }
  pc = 0x6125be5; continue;
 }
 0x6125be5 => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125beb; continue;
 }
 0x6125beb => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125c2f; continue;
 }
 0x6125bfe => {
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
  pc = 0x6125c2f; continue;
 }
 0x6125c2f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125bfe; continue; }
  pc = 0x6125c37; continue;
 }
 0x6125c37 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x46u32; cpu.ebp = v; }
  pc = 0x6125c3d; continue;
 }
 0x6125c3d => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x6300034u32)); }
  if cpu.cf { pc = 0x6125c51; continue; }
  pc = 0x6125c4b; continue;
 }
 0x6125c4b => {
  { let v: u32 = 0x6250ee0u32; cpu.edx = v; }
  pc = 0x6125c51; continue;
 }
 0x6125c51 => {
  { let v: u32 = cpu.r32(0x6173fd8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6300018u32, v); }
  pc = 0x6125c95; continue;
 }
 0x6125c64 => {
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
  pc = 0x6125c95; continue;
 }
 0x6125c95 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x6300018u32)); }
  if cpu.cf { cpu.poll(); pc = 0x6125c64; continue; }
  pc = 0x6125c9d; continue;
 }
 0x6125c9d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6125c3d; continue; }
  pc = 0x6125ca1; continue;
 }
 0x6125ca1 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125ca4); }
 let mut pc: u32 = 0x6125ca4;
 loop { match pc {
 0x6125ca4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125cac); }
 let mut pc: u32 = 0x6125cac;
 loop { match pc {
 0x6125cac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125cb4); }
 let mut pc: u32 = 0x6125cb4;
 loop { match pc {
 0x6125cb4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x6125cbd; continue;
 }
 0x6125cbd => {
  pc = 0x6125cbf; continue;
 }
 0x6125cbf => {
  pc = 0x6125cc1; continue;
 }
 0x6125cc1 => {
  {  }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x6125cc3; continue;
 }
 0x6125cc3 => {
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x6125ccc; continue;
 }
 0x6125ccc => {
  pc = 0x6125cce; continue;
 }
 0x6125cce => {
  pc = 0x6125cd0; continue;
 }
 0x6125cd0 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x6125cde; continue;
 }
 0x6125cde => {
  pc = 0x6125ce0; continue;
 }
 0x6125ce0 => {
  pc = 0x6125ce2; continue;
 }
 0x6125ce2 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x6125cf0; continue;
 }
 0x6125cf0 => {
  pc = 0x6125cf2; continue;
 }
 0x6125cf2 => {
  pc = 0x6125cf4; continue;
 }
 0x6125cf4 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x6125cc3; continue; }
  pc = 0x6125cfc; continue;
 }
 0x6125cfc => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125d12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125d12); }
 let mut pc: u32 = 0x6125d12;
 loop { match pc {
 0x6125d12 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x6125d20; continue; }
  pc = 0x6125d1b; continue;
 }
 0x6125d1b => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x6125d1b; continue; }
  pc = 0x6125d20; continue;
 }
 0x6125d20 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x6125d20; continue; }
  pc = 0x6125d25; continue;
 }
 0x6125d25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125d26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125d26); }
 let mut pc: u32 = 0x6125d26;
 loop { match pc {
 0x6125d26 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x6125d34; continue; }
  pc = 0x6125d2f; continue;
 }
 0x6125d2f => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x6125d2f; continue; }
  pc = 0x6125d34; continue;
 }
 0x6125d34 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x6125d34; continue; }
  pc = 0x6125d39; continue;
 }
 0x6125d39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125d3a); }
 let mut pc: u32 = 0x6125d3a;
 loop { match pc {
 0x6125d3a => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62519a0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06125d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6125d47); }
 let mut pc: u32 = 0x6125d47;
 loop { match pc {
 0x6125d47 => {
  { let v: u16 = cpu.r16(0x62519a0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x625199au32), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x6125d83; continue; }
  pc = 0x6125d57; continue;
 }
 0x6125d57 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x625199au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x625199cu32), 0xffffu16); }
  if cpu.zf { pc = 0x6125d83; continue; }
  pc = 0x6125d72; continue;
 }
 0x6125d72 => {
  { let v: u16 = cpu.r16(0x625199au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x6125d83; continue;
 }
 0x6125d83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06130000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6130000); }
 let mut pc: u32 = 0x6130000;
 loop { match pc {
 0x6130000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x6130003; continue;
 }
 0x6130003 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x6130011; continue; }
  pc = 0x613000e; continue;
 }
 0x613000e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x6130011; continue;
 }
 0x6130011 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x6130003; continue; }
  pc = 0x6130020; continue;
 }
 0x6130020 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06130022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6130022); }
 let mut pc: u32 = 0x6130022;
 loop { match pc {
 0x6130022 => {
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
pub fn f_06130038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6130038); }
 let mut pc: u32 = 0x6130038;
 loop { match pc {
 0x6130038 => {
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
  { let v: u32 = 0x0u32; cpu.w32(0x6250e7cu32, v); }
  pc = 0x6130064; continue;
 }
 0x6130064 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x6250e7cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x6130080; continue; }
  pc = 0x6130071; continue;
 }
 0x6130071 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x6250e7cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6250e7cu32, v); }
  pc = 0x6130080; continue;
 }
 0x6130080 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x6130085; continue; }
  pc = 0x6130084; continue;
 }
 0x6130084 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x6130085; continue;
 }
 0x6130085 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x613008a; continue; }
  pc = 0x6130089; continue;
 }
 0x6130089 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x613008a; continue;
 }
 0x613008a => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x613008f; continue; }
  pc = 0x613008e; continue;
 }
 0x613008e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x613008f; continue;
 }
 0x613008f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x6130094; continue; }
  pc = 0x6130093; continue;
 }
 0x6130093 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x6130094; continue;
 }
 0x6130094 => {
  { cpu.and8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x6130064; continue; }
  pc = 0x6130098; continue;
 }
 0x6130098 => {
  {  }
  { let v: u32 = cpu.esi; cpu.w32(0x6250ea0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x6250e9cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6250ea4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x6250e80u32, v); }
  { let v: u8 = cpu.r8(0x6250e7cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6250e7cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06140000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x6140000); } r_06140000(cpu, 0x6140000); }
pub fn f_06140008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x6140008); } r_06140000(cpu, 0x6140008); }
fn r_06140000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x6140000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x6140008; continue;
 }
 0x6140008 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.esi; cpu.set_segment(3, v as u16); }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if !cpu.zf { pc = 0x6140014; continue; }
  pc = 0x6140011; continue;
 }
 0x6140011 => {
  cpu.flat_ds(0x6140011); cpu.push(0x6140016); cpu.call(0x5970051); if cpu.returned_to != 0x6140016 { return; }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esi); cpu.w32(0x3u32.wrapping_add(cpu.ebp), v); }
  pc = 0x6140019; continue;
 }
 0x6140014 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x614001c; continue; }
  pc = 0x6140019; continue;
 }
 0x6140019 => {
  cpu.flat_ds(0x6140019); cpu.push(0x614001e); cpu.call(0x5970065); if cpu.returned_to != 0x614001e { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x3u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x6140021; continue;
 }
 0x614001c => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x6140024; continue; }
  pc = 0x6140021; continue;
 }
 0x6140021 => {
  cpu.flat_ds(0x6140021); cpu.push(0x6140026); cpu.call(0x5970073); if cpu.returned_to != 0x6140026 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  pc = 0x6140029; continue;
 }
 0x6140024 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x614002c; continue; }
  pc = 0x6140029; continue;
 }
 0x6140029 => {
  cpu.flat_ds(0x6140029); cpu.push(0x614002e); cpu.call(0x5970079); if cpu.returned_to != 0x614002e { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x75u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x6140039; continue;
 }
 0x614002c => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x6140034; continue; }
  pc = 0x6140031; continue;
 }
 0x6140031 => {
  cpu.flat_ds(0x6140031); cpu.push(0x6140036); cpu.call(0x597008d); if cpu.returned_to != 0x6140036 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x55e80375u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x6140041; continue;
 }
 0x6140034 => {
  { cpu.sub32(cpu.edi, 0x5u32); }
  if !cpu.zf { pc = 0x614003c; continue; }
  pc = 0x6140039; continue;
 }
 0x6140039 => {
  cpu.flat_ds(0x6140039); cpu.push(0x614003e); cpu.call(0x5970093); if cpu.returned_to != 0x614003e { return; }
  { cpu.push(cpu.segments[0] as u32); }
  if !cpu.zf { pc = 0x6140044; continue; }
  pc = 0x6140041; continue;
 }
 0x614003c => {
  { cpu.sub32(cpu.edi, 0x6u32); }
  if !cpu.zf { pc = 0x6140044; continue; }
  pc = 0x6140041; continue;
 }
 0x6140041 => {
  cpu.flat_ds(0x6140041); cpu.push(0x6140046); cpu.call(0x59700a3); if cpu.returned_to != 0x6140046 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if !cpu.zf { pc = 0x614004c; continue; }
  pc = 0x6140049; continue;
 }
 0x6140044 => {
  { cpu.sub32(cpu.edi, 0x7u32); }
  if !cpu.zf { pc = 0x614004c; continue; }
  pc = 0x6140049; continue;
 }
 0x6140049 => {
  cpu.flat_ds(0x6140049); cpu.push(0x614004e); cpu.call(0xd3300b1); if cpu.returned_to != 0x614004e { return; }
  cpu.flat_ds(0x614004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x614004c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x614004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0614004f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x614004f); }
 let mut pc: u32 = 0x614004f;
 loop { match pc {
 0x614004f => {
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
pub fn f_06150000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x6150000); } r_06150000(cpu, 0x6150000); }
pub fn f_06150002(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x6150002); } r_06150000(cpu, 0x6150002); }
fn r_06150000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x6150000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x6150002; continue;
 }
 0x6150002 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x6150042; continue; }
  pc = 0x6150009; continue;
 }
 0x6150009 => {
  { let v: u32 = 0x66670007u32; cpu.eax = v; }
  { let v: u32 = 0xffffe000u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 16u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x6150042; continue; }
  pc = 0x615001f; continue;
 }
 0x615001f => {
  {  }
  { let v = cpu.ebx; cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x8dfb2000u32; cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0x500e0fffu32); cpu.eax = v; }
  { let v = (cpu.edi as u16); cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x6150036 => {
  {  }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.ebp as u16); cpu.esp = (cpu.esp & 0xffff0000) | v as u32; }
  {  }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x6150042 => {
  cpu.unimplemented(0x6150042, "Int3");
  { let v: u16 = (cpu.esi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x6150036; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_06000000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x6000000); }
 let mut pc: u32 = 0x6000000;
 loop { match pc {
 0x6000000 => {
  cpu.unimplemented(0x6000000, "Int3");
  cpu.poll(); pc = 0x6000000; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bf526(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bf526); }
 let mut pc: u32 = 0x60bf526;
 loop { match pc {
 0x60bf521 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  pc = 0x60bf526; continue;
 }
 0x60bf526 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.of { cpu.poll(); pc = 0x60bf521; continue; }
  pc = 0x60bf52c; continue;
 }
 0x60bf52c => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0xf5u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.call(0x60bf532); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2164(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2164); }
 let mut pc: u32 = 0x60c2164;
 loop { match pc {
 0x60c2164 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2164; continue; }
  pc = 0x60c2174; continue;
 }
 0x60c2174 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2175(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2175); }
 let mut pc: u32 = 0x60c2175;
 loop { match pc {
 0x60c2175 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2175; continue; }
  pc = 0x60c2180; continue;
 }
 0x60c2180 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2181(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2181); }
 let mut pc: u32 = 0x60c2181;
 loop { match pc {
 0x60c2181 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2181; continue; }
  pc = 0x60c219f; continue;
 }
 0x60c219f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c21a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c21a0); }
 let mut pc: u32 = 0x60c21a0;
 loop { match pc {
 0x60c21a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c21a0; continue; }
  pc = 0x60c21b3; continue;
 }
 0x60c21b3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c21b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c21b4); }
 let mut pc: u32 = 0x60c21b4;
 loop { match pc {
 0x60c21b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c21b4; continue; }
  pc = 0x60c21d2; continue;
 }
 0x60c21d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c21d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c21d3); }
 let mut pc: u32 = 0x60c21d3;
 loop { match pc {
 0x60c21d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c21d3; continue; }
  pc = 0x60c21e6; continue;
 }
 0x60c21e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c21e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c21e7); }
 let mut pc: u32 = 0x60c21e7;
 loop { match pc {
 0x60c21e7 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c21e7; continue; }
  pc = 0x60c2203; continue;
 }
 0x60c2203 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2204(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2204); }
 let mut pc: u32 = 0x60c2204;
 loop { match pc {
 0x60c2204 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2204; continue; }
  pc = 0x60c2215; continue;
 }
 0x60c2215 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2216(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2216); }
 let mut pc: u32 = 0x60c2216;
 loop { match pc {
 0x60c2216 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c2216; continue; }
  pc = 0x60c2232; continue;
 }
 0x60c2232 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2233(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2233); }
 let mut pc: u32 = 0x60c2233;
 loop { match pc {
 0x60c2233 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2233; continue; }
  pc = 0x60c2244; continue;
 }
 0x60c2244 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2245(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2245); }
 let mut pc: u32 = 0x60c2245;
 loop { match pc {
 0x60c2245 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2245; continue; }
  pc = 0x60c2258; continue;
 }
 0x60c2258 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2259(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2259); }
 let mut pc: u32 = 0x60c2259;
 loop { match pc {
 0x60c2259 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2259; continue; }
  pc = 0x60c2267; continue;
 }
 0x60c2267 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2268); }
 let mut pc: u32 = 0x60c2268;
 loop { match pc {
 0x60c2268 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2268; continue; }
  pc = 0x60c227d; continue;
 }
 0x60c227d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c227e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c227e); }
 let mut pc: u32 = 0x60c227e;
 loop { match pc {
 0x60c227e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c227e; continue; }
  pc = 0x60c228e; continue;
 }
 0x60c228e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c228f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c228f); }
 let mut pc: u32 = 0x60c228f;
 loop { match pc {
 0x60c228f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c228f; continue; }
  pc = 0x60c22af; continue;
 }
 0x60c22af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c22b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c22b0); }
 let mut pc: u32 = 0x60c22b0;
 loop { match pc {
 0x60c22b0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c22b0; continue; }
  pc = 0x60c22c5; continue;
 }
 0x60c22c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c22c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c22c6); }
 let mut pc: u32 = 0x60c22c6;
 loop { match pc {
 0x60c22c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c22c6; continue; }
  pc = 0x60c22e6; continue;
 }
 0x60c22e6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c22e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c22e7); }
 let mut pc: u32 = 0x60c22e7;
 loop { match pc {
 0x60c22e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c22e7; continue; }
  pc = 0x60c22fc; continue;
 }
 0x60c22fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c22fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c22fd); }
 let mut pc: u32 = 0x60c22fd;
 loop { match pc {
 0x60c22fd => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c22fd; continue; }
  pc = 0x60c2321; continue;
 }
 0x60c2321 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2322(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2322); }
 let mut pc: u32 = 0x60c2322;
 loop { match pc {
 0x60c2322 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2322; continue; }
  pc = 0x60c233b; continue;
 }
 0x60c233b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c233c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c233c); }
 let mut pc: u32 = 0x60c233c;
 loop { match pc {
 0x60c233c => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c233c; continue; }
  pc = 0x60c2360; continue;
 }
 0x60c2360 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2361(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2361); }
 let mut pc: u32 = 0x60c2361;
 loop { match pc {
 0x60c2361 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2361; continue; }
  pc = 0x60c237a; continue;
 }
 0x60c237a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c237b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c237b); }
 let mut pc: u32 = 0x60c237b;
 loop { match pc {
 0x60c237b => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c237b; continue; }
  pc = 0x60c239d; continue;
 }
 0x60c239d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c239e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c239e); }
 let mut pc: u32 = 0x60c239e;
 loop { match pc {
 0x60c239e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c239e; continue; }
  pc = 0x60c23b5; continue;
 }
 0x60c23b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c23b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c23b6); }
 let mut pc: u32 = 0x60c23b6;
 loop { match pc {
 0x60c23b6 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c23b6; continue; }
  pc = 0x60c23d8; continue;
 }
 0x60c23d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c23d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c23d9); }
 let mut pc: u32 = 0x60c23d9;
 loop { match pc {
 0x60c23d9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c23d9; continue; }
  pc = 0x60c23f0; continue;
 }
 0x60c23f0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c23f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c23f1); }
 let mut pc: u32 = 0x60c23f1;
 loop { match pc {
 0x60c23f1 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c23f1; continue; }
  pc = 0x60c2417; continue;
 }
 0x60c2417 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2418(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2418); }
 let mut pc: u32 = 0x60c2418;
 loop { match pc {
 0x60c2418 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c2418; continue; }
  pc = 0x60c2433; continue;
 }
 0x60c2433 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2434(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2434); }
 let mut pc: u32 = 0x60c2434;
 loop { match pc {
 0x60c2434 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c2434; continue; }
  pc = 0x60c245a; continue;
 }
 0x60c245a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c245b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c245b); }
 let mut pc: u32 = 0x60c245b;
 loop { match pc {
 0x60c245b => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c245b; continue; }
  pc = 0x60c2476; continue;
 }
 0x60c2476 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2477(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2477); }
 let mut pc: u32 = 0x60c2477;
 loop { match pc {
 0x60c2477 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2496; continue; }
  pc = 0x60c2483; continue;
 }
 0x60c2483 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c249d; continue; }
  pc = 0x60c248a; continue;
 }
 0x60c248a => {
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2477; continue; }
  pc = 0x60c2495; continue;
 }
 0x60c2495 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2496 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c248a; continue;
 }
 0x60c249d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c248a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c24a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c24a4); }
 let mut pc: u32 = 0x60c24a4;
 loop { match pc {
 0x60c24a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c24be; continue; }
  pc = 0x60c24b0; continue;
 }
 0x60c24b0 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c24c5; continue; }
  pc = 0x60c24b7; continue;
 }
 0x60c24b7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c24a4; continue; }
  pc = 0x60c24bd; continue;
 }
 0x60c24bd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c24be => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c24b7; continue;
 }
 0x60c24c5 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c24b7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c24cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c24cc); }
 let mut pc: u32 = 0x60c24cc;
 loop { match pc {
 0x60c24cc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2509; continue; }
  pc = 0x60c24db; continue;
 }
 0x60c24db => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2510; continue; }
  pc = 0x60c24e2; continue;
 }
 0x60c24e2 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2517; continue; }
  pc = 0x60c24ea; continue;
 }
 0x60c24ea => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c251e; continue; }
  pc = 0x60c24f2; continue;
 }
 0x60c24f2 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c24cc; continue; }
  pc = 0x60c2508; continue;
 }
 0x60c2508 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2509 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c24e2; continue;
 }
 0x60c2510 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c24e2; continue;
 }
 0x60c2517 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c24f2; continue;
 }
 0x60c251e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c24f2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2525(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2525); }
 let mut pc: u32 = 0x60c2525;
 loop { match pc {
 0x60c2525 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2557; continue; }
  pc = 0x60c2534; continue;
 }
 0x60c2534 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c255e; continue; }
  pc = 0x60c253b; continue;
 }
 0x60c253b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2565; continue; }
  pc = 0x60c2543; continue;
 }
 0x60c2543 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c256c; continue; }
  pc = 0x60c254b; continue;
 }
 0x60c254b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2525; continue; }
  pc = 0x60c2556; continue;
 }
 0x60c2556 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2557 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c253b; continue;
 }
 0x60c255e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c253b; continue;
 }
 0x60c2565 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c254b; continue;
 }
 0x60c256c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c254b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2573(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2573); }
 let mut pc: u32 = 0x60c2573;
 loop { match pc {
 0x60c2573 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c25b0; continue; }
  pc = 0x60c2582; continue;
 }
 0x60c2582 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c25b7; continue; }
  pc = 0x60c2589; continue;
 }
 0x60c2589 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c25be; continue; }
  pc = 0x60c2591; continue;
 }
 0x60c2591 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c25c5; continue; }
  pc = 0x60c2599; continue;
 }
 0x60c2599 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2573; continue; }
  pc = 0x60c25af; continue;
 }
 0x60c25af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c25b0 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2589; continue;
 }
 0x60c25b7 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2589; continue;
 }
 0x60c25be => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2599; continue;
 }
 0x60c25c5 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2599; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c25cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c25cc); }
 let mut pc: u32 = 0x60c25cc;
 loop { match pc {
 0x60c25cc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c25fe; continue; }
  pc = 0x60c25db; continue;
 }
 0x60c25db => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2605; continue; }
  pc = 0x60c25e2; continue;
 }
 0x60c25e2 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c260c; continue; }
  pc = 0x60c25ea; continue;
 }
 0x60c25ea => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2613; continue; }
  pc = 0x60c25f2; continue;
 }
 0x60c25f2 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c25cc; continue; }
  pc = 0x60c25fd; continue;
 }
 0x60c25fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c25fe => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c25e2; continue;
 }
 0x60c2605 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c25e2; continue;
 }
 0x60c260c => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c25f2; continue;
 }
 0x60c2613 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c25f2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c261a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c261a); }
 let mut pc: u32 = 0x60c261a;
 loop { match pc {
 0x60c261a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2655; continue; }
  pc = 0x60c2629; continue;
 }
 0x60c2629 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c265c; continue; }
  pc = 0x60c2630; continue;
 }
 0x60c2630 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2663; continue; }
  pc = 0x60c2638; continue;
 }
 0x60c2638 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c266a; continue; }
  pc = 0x60c2640; continue;
 }
 0x60c2640 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c261a; continue; }
  pc = 0x60c2654; continue;
 }
 0x60c2654 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2655 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2630; continue;
 }
 0x60c265c => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2630; continue;
 }
 0x60c2663 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2640; continue;
 }
 0x60c266a => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2640; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2671); }
 let mut pc: u32 = 0x60c2671;
 loop { match pc {
 0x60c2671 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c26a1; continue; }
  pc = 0x60c2680; continue;
 }
 0x60c2680 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c26a8; continue; }
  pc = 0x60c2687; continue;
 }
 0x60c2687 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c26af; continue; }
  pc = 0x60c268f; continue;
 }
 0x60c268f => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c26b6; continue; }
  pc = 0x60c2697; continue;
 }
 0x60c2697 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2671; continue; }
  pc = 0x60c26a0; continue;
 }
 0x60c26a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c26a1 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2687; continue;
 }
 0x60c26a8 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2687; continue;
 }
 0x60c26af => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2697; continue;
 }
 0x60c26b6 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2697; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c26bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c26bd); }
 let mut pc: u32 = 0x60c26bd;
 loop { match pc {
 0x60c26bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c26f8; continue; }
  pc = 0x60c26cc; continue;
 }
 0x60c26cc => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c26ff; continue; }
  pc = 0x60c26d3; continue;
 }
 0x60c26d3 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2706; continue; }
  pc = 0x60c26db; continue;
 }
 0x60c26db => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c270d; continue; }
  pc = 0x60c26e3; continue;
 }
 0x60c26e3 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c26bd; continue; }
  pc = 0x60c26f7; continue;
 }
 0x60c26f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c26f8 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c26d3; continue;
 }
 0x60c26ff => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c26d3; continue;
 }
 0x60c2706 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c26e3; continue;
 }
 0x60c270d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c26e3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2714(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2714); }
 let mut pc: u32 = 0x60c2714;
 loop { match pc {
 0x60c2714 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2744; continue; }
  pc = 0x60c2723; continue;
 }
 0x60c2723 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c274b; continue; }
  pc = 0x60c272a; continue;
 }
 0x60c272a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2752; continue; }
  pc = 0x60c2732; continue;
 }
 0x60c2732 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2759; continue; }
  pc = 0x60c273a; continue;
 }
 0x60c273a => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2714; continue; }
  pc = 0x60c2743; continue;
 }
 0x60c2743 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2744 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c272a; continue;
 }
 0x60c274b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c272a; continue;
 }
 0x60c2752 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c273a; continue;
 }
 0x60c2759 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c273a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2760); }
 let mut pc: u32 = 0x60c2760;
 loop { match pc {
 0x60c2760 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2782; continue; }
  pc = 0x60c276c; continue;
 }
 0x60c276c => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2789; continue; }
  pc = 0x60c2773; continue;
 }
 0x60c2773 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2760; continue; }
  pc = 0x60c2781; continue;
 }
 0x60c2781 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2782 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2773; continue;
 }
 0x60c2789 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2773; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2790); }
 let mut pc: u32 = 0x60c2790;
 loop { match pc {
 0x60c2790 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c27ad; continue; }
  pc = 0x60c279c; continue;
 }
 0x60c279c => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c27b4; continue; }
  pc = 0x60c27a3; continue;
 }
 0x60c27a3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2790; continue; }
  pc = 0x60c27ac; continue;
 }
 0x60c27ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c27ad => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c27a3; continue;
 }
 0x60c27b4 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c27a3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c27bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c27bb); }
 let mut pc: u32 = 0x60c27bb;
 loop { match pc {
 0x60c27bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c27df; continue; }
  pc = 0x60c27c7; continue;
 }
 0x60c27c7 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c27e6; continue; }
  pc = 0x60c27ce; continue;
 }
 0x60c27ce => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c27bb; continue; }
  pc = 0x60c27de; continue;
 }
 0x60c27de => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c27df => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c27ce; continue;
 }
 0x60c27e6 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c27ce; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c27ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c27ed); }
 let mut pc: u32 = 0x60c27ed;
 loop { match pc {
 0x60c27ed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c280c; continue; }
  pc = 0x60c27f9; continue;
 }
 0x60c27f9 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2813; continue; }
  pc = 0x60c2800; continue;
 }
 0x60c2800 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c27ed; continue; }
  pc = 0x60c280b; continue;
 }
 0x60c280b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c280c => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2800; continue;
 }
 0x60c2813 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2800; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c281a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c281a); }
 let mut pc: u32 = 0x60c281a;
 loop { match pc {
 0x60c281a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2859; continue; }
  pc = 0x60c2829; continue;
 }
 0x60c2829 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2860; continue; }
  pc = 0x60c2830; continue;
 }
 0x60c2830 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2867; continue; }
  pc = 0x60c2838; continue;
 }
 0x60c2838 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c286e; continue; }
  pc = 0x60c2840; continue;
 }
 0x60c2840 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c281a; continue; }
  pc = 0x60c2858; continue;
 }
 0x60c2858 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2859 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2830; continue;
 }
 0x60c2860 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2830; continue;
 }
 0x60c2867 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2840; continue;
 }
 0x60c286e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2840; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2875(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2875); }
 let mut pc: u32 = 0x60c2875;
 loop { match pc {
 0x60c2875 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c28a9; continue; }
  pc = 0x60c2884; continue;
 }
 0x60c2884 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c28b0; continue; }
  pc = 0x60c288b; continue;
 }
 0x60c288b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c28b7; continue; }
  pc = 0x60c2893; continue;
 }
 0x60c2893 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c28be; continue; }
  pc = 0x60c289b; continue;
 }
 0x60c289b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2875; continue; }
  pc = 0x60c28a8; continue;
 }
 0x60c28a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c28a9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c288b; continue;
 }
 0x60c28b0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c288b; continue;
 }
 0x60c28b7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c289b; continue;
 }
 0x60c28be => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c289b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c28c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c28c5); }
 let mut pc: u32 = 0x60c28c5;
 loop { match pc {
 0x60c28c5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2904; continue; }
  pc = 0x60c28d4; continue;
 }
 0x60c28d4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c290b; continue; }
  pc = 0x60c28db; continue;
 }
 0x60c28db => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2912; continue; }
  pc = 0x60c28e3; continue;
 }
 0x60c28e3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2919; continue; }
  pc = 0x60c28eb; continue;
 }
 0x60c28eb => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c28c5; continue; }
  pc = 0x60c2903; continue;
 }
 0x60c2903 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2904 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c28db; continue;
 }
 0x60c290b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c28db; continue;
 }
 0x60c2912 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c28eb; continue;
 }
 0x60c2919 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c28eb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2920); }
 let mut pc: u32 = 0x60c2920;
 loop { match pc {
 0x60c2920 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2954; continue; }
  pc = 0x60c292f; continue;
 }
 0x60c292f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c295b; continue; }
  pc = 0x60c2936; continue;
 }
 0x60c2936 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2962; continue; }
  pc = 0x60c293e; continue;
 }
 0x60c293e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2969; continue; }
  pc = 0x60c2946; continue;
 }
 0x60c2946 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2920; continue; }
  pc = 0x60c2953; continue;
 }
 0x60c2953 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2954 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2936; continue;
 }
 0x60c295b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2936; continue;
 }
 0x60c2962 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2946; continue;
 }
 0x60c2969 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2946; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2970); }
 let mut pc: u32 = 0x60c2970;
 loop { match pc {
 0x60c2970 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c29b3; continue; }
  pc = 0x60c297f; continue;
 }
 0x60c297f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c29ba; continue; }
  pc = 0x60c2986; continue;
 }
 0x60c2986 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c29c1; continue; }
  pc = 0x60c298e; continue;
 }
 0x60c298e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c29c8; continue; }
  pc = 0x60c2996; continue;
 }
 0x60c2996 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2970; continue; }
  pc = 0x60c29b2; continue;
 }
 0x60c29b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c29b3 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2986; continue;
 }
 0x60c29ba => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2986; continue;
 }
 0x60c29c1 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2996; continue;
 }
 0x60c29c8 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2996; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c29cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c29cf); }
 let mut pc: u32 = 0x60c29cf;
 loop { match pc {
 0x60c29cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2a07; continue; }
  pc = 0x60c29de; continue;
 }
 0x60c29de => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2a0e; continue; }
  pc = 0x60c29e5; continue;
 }
 0x60c29e5 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2a15; continue; }
  pc = 0x60c29ed; continue;
 }
 0x60c29ed => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2a1c; continue; }
  pc = 0x60c29f5; continue;
 }
 0x60c29f5 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c29cf; continue; }
  pc = 0x60c2a06; continue;
 }
 0x60c2a06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2a07 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c29e5; continue;
 }
 0x60c2a0e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c29e5; continue;
 }
 0x60c2a15 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c29f5; continue;
 }
 0x60c2a1c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c29f5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2a23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2a23); }
 let mut pc: u32 = 0x60c2a23;
 loop { match pc {
 0x60c2a23 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2a66; continue; }
  pc = 0x60c2a32; continue;
 }
 0x60c2a32 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2a6d; continue; }
  pc = 0x60c2a39; continue;
 }
 0x60c2a39 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2a74; continue; }
  pc = 0x60c2a41; continue;
 }
 0x60c2a41 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2a7b; continue; }
  pc = 0x60c2a49; continue;
 }
 0x60c2a49 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2a23; continue; }
  pc = 0x60c2a65; continue;
 }
 0x60c2a65 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2a66 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2a39; continue;
 }
 0x60c2a6d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2a39; continue;
 }
 0x60c2a74 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2a49; continue;
 }
 0x60c2a7b => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2a49; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2a82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2a82); }
 let mut pc: u32 = 0x60c2a82;
 loop { match pc {
 0x60c2a82 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2aba; continue; }
  pc = 0x60c2a91; continue;
 }
 0x60c2a91 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2ac1; continue; }
  pc = 0x60c2a98; continue;
 }
 0x60c2a98 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2ac8; continue; }
  pc = 0x60c2aa0; continue;
 }
 0x60c2aa0 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2acf; continue; }
  pc = 0x60c2aa8; continue;
 }
 0x60c2aa8 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2a82; continue; }
  pc = 0x60c2ab9; continue;
 }
 0x60c2ab9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2aba => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2a98; continue;
 }
 0x60c2ac1 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2a98; continue;
 }
 0x60c2ac8 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2aa8; continue;
 }
 0x60c2acf => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2aa8; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2ad6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2ad6); }
 let mut pc: u32 = 0x60c2ad6;
 loop { match pc {
 0x60c2ad6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2b17; continue; }
  pc = 0x60c2ae5; continue;
 }
 0x60c2ae5 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2b1e; continue; }
  pc = 0x60c2aec; continue;
 }
 0x60c2aec => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2b25; continue; }
  pc = 0x60c2af4; continue;
 }
 0x60c2af4 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2b2c; continue; }
  pc = 0x60c2afc; continue;
 }
 0x60c2afc => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2ad6; continue; }
  pc = 0x60c2b16; continue;
 }
 0x60c2b16 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2b17 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2aec; continue;
 }
 0x60c2b1e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2aec; continue;
 }
 0x60c2b25 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2afc; continue;
 }
 0x60c2b2c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2afc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2b33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2b33); }
 let mut pc: u32 = 0x60c2b33;
 loop { match pc {
 0x60c2b33 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2b69; continue; }
  pc = 0x60c2b42; continue;
 }
 0x60c2b42 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2b70; continue; }
  pc = 0x60c2b49; continue;
 }
 0x60c2b49 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2b77; continue; }
  pc = 0x60c2b51; continue;
 }
 0x60c2b51 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2b7e; continue; }
  pc = 0x60c2b59; continue;
 }
 0x60c2b59 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2b33; continue; }
  pc = 0x60c2b68; continue;
 }
 0x60c2b68 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2b69 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2b49; continue;
 }
 0x60c2b70 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2b49; continue;
 }
 0x60c2b77 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2b59; continue;
 }
 0x60c2b7e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2b59; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2b85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2b85); }
 let mut pc: u32 = 0x60c2b85;
 loop { match pc {
 0x60c2b85 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2bc6; continue; }
  pc = 0x60c2b94; continue;
 }
 0x60c2b94 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2bcd; continue; }
  pc = 0x60c2b9b; continue;
 }
 0x60c2b9b => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2bd4; continue; }
  pc = 0x60c2ba3; continue;
 }
 0x60c2ba3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2bdb; continue; }
  pc = 0x60c2bab; continue;
 }
 0x60c2bab => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2b85; continue; }
  pc = 0x60c2bc5; continue;
 }
 0x60c2bc5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2bc6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2b9b; continue;
 }
 0x60c2bcd => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2b9b; continue;
 }
 0x60c2bd4 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2bab; continue;
 }
 0x60c2bdb => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2bab; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2be2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2be2); }
 let mut pc: u32 = 0x60c2be2;
 loop { match pc {
 0x60c2be2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2c18; continue; }
  pc = 0x60c2bf1; continue;
 }
 0x60c2bf1 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2c1f; continue; }
  pc = 0x60c2bf8; continue;
 }
 0x60c2bf8 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2c26; continue; }
  pc = 0x60c2c00; continue;
 }
 0x60c2c00 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2c2d; continue; }
  pc = 0x60c2c08; continue;
 }
 0x60c2c08 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2be2; continue; }
  pc = 0x60c2c17; continue;
 }
 0x60c2c17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2c18 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2bf8; continue;
 }
 0x60c2c1f => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2bf8; continue;
 }
 0x60c2c26 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2c08; continue;
 }
 0x60c2c2d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2c08; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2c34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2c34); }
 let mut pc: u32 = 0x60c2c34;
 loop { match pc {
 0x60c2c34 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2c79; continue; }
  pc = 0x60c2c43; continue;
 }
 0x60c2c43 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2c80; continue; }
  pc = 0x60c2c4a; continue;
 }
 0x60c2c4a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2c87; continue; }
  pc = 0x60c2c52; continue;
 }
 0x60c2c52 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2c8e; continue; }
  pc = 0x60c2c5a; continue;
 }
 0x60c2c5a => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2c34; continue; }
  pc = 0x60c2c78; continue;
 }
 0x60c2c78 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2c79 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2c4a; continue;
 }
 0x60c2c80 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2c4a; continue;
 }
 0x60c2c87 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2c5a; continue;
 }
 0x60c2c8e => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2c5a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2c95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2c95); }
 let mut pc: u32 = 0x60c2c95;
 loop { match pc {
 0x60c2c95 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2ccf; continue; }
  pc = 0x60c2ca4; continue;
 }
 0x60c2ca4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2cd6; continue; }
  pc = 0x60c2cab; continue;
 }
 0x60c2cab => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2cdd; continue; }
  pc = 0x60c2cb3; continue;
 }
 0x60c2cb3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2ce4; continue; }
  pc = 0x60c2cbb; continue;
 }
 0x60c2cbb => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2c95; continue; }
  pc = 0x60c2cce; continue;
 }
 0x60c2cce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2ccf => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2cab; continue;
 }
 0x60c2cd6 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2cab; continue;
 }
 0x60c2cdd => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2cbb; continue;
 }
 0x60c2ce4 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2cbb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2ceb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2ceb); }
 let mut pc: u32 = 0x60c2ceb;
 loop { match pc {
 0x60c2ceb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2d30; continue; }
  pc = 0x60c2cfa; continue;
 }
 0x60c2cfa => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2d37; continue; }
  pc = 0x60c2d01; continue;
 }
 0x60c2d01 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2d3e; continue; }
  pc = 0x60c2d09; continue;
 }
 0x60c2d09 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2d45; continue; }
  pc = 0x60c2d11; continue;
 }
 0x60c2d11 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2ceb; continue; }
  pc = 0x60c2d2f; continue;
 }
 0x60c2d2f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2d30 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2d01; continue;
 }
 0x60c2d37 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2d01; continue;
 }
 0x60c2d3e => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2d11; continue;
 }
 0x60c2d45 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2d11; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2d4c); }
 let mut pc: u32 = 0x60c2d4c;
 loop { match pc {
 0x60c2d4c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2d86; continue; }
  pc = 0x60c2d5b; continue;
 }
 0x60c2d5b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2d8d; continue; }
  pc = 0x60c2d62; continue;
 }
 0x60c2d62 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c2d94; continue; }
  pc = 0x60c2d6a; continue;
 }
 0x60c2d6a => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c2d9b; continue; }
  pc = 0x60c2d72; continue;
 }
 0x60c2d72 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c2d4c; continue; }
  pc = 0x60c2d85; continue;
 }
 0x60c2d85 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c2d86 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2d62; continue;
 }
 0x60c2d8d => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c2d62; continue;
 }
 0x60c2d94 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2d72; continue;
 }
 0x60c2d9b => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c2d72; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2da2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2da2); }
 let mut pc: u32 = 0x60c2da2;
 loop { match pc {
 0x60c2da2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2dc2; continue; }
  pc = 0x60c2daa; continue;
 }
 0x60c2daa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2da2; continue; }
  pc = 0x60c2dc2; continue;
 }
 0x60c2dc2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2dc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2dc3); }
 let mut pc: u32 = 0x60c2dc3;
 loop { match pc {
 0x60c2dc3 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2de6; continue; }
  pc = 0x60c2dcb; continue;
 }
 0x60c2dcb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2dc3; continue; }
  pc = 0x60c2de6; continue;
 }
 0x60c2de6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2de7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2de7); }
 let mut pc: u32 = 0x60c2de7;
 loop { match pc {
 0x60c2de7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2e18; continue; }
  pc = 0x60c2def; continue;
 }
 0x60c2def => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2de7; continue; }
  pc = 0x60c2e18; continue;
 }
 0x60c2e18 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2e19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2e19); }
 let mut pc: u32 = 0x60c2e19;
 loop { match pc {
 0x60c2e19 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2e4a; continue; }
  pc = 0x60c2e21; continue;
 }
 0x60c2e21 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2e19; continue; }
  pc = 0x60c2e4a; continue;
 }
 0x60c2e4a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2e4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2e4b); }
 let mut pc: u32 = 0x60c2e4b;
 loop { match pc {
 0x60c2e4b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2e6e; continue; }
  pc = 0x60c2e53; continue;
 }
 0x60c2e53 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2e4b; continue; }
  pc = 0x60c2e6e; continue;
 }
 0x60c2e6e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2e6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2e6f); }
 let mut pc: u32 = 0x60c2e6f;
 loop { match pc {
 0x60c2e6f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2e95; continue; }
  pc = 0x60c2e77; continue;
 }
 0x60c2e77 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2e6f; continue; }
  pc = 0x60c2e95; continue;
 }
 0x60c2e95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2e96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2e96); }
 let mut pc: u32 = 0x60c2e96;
 loop { match pc {
 0x60c2e96 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2ec9; continue; }
  pc = 0x60c2e9e; continue;
 }
 0x60c2e9e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2e96; continue; }
  pc = 0x60c2ec9; continue;
 }
 0x60c2ec9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2eca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2eca); }
 let mut pc: u32 = 0x60c2eca;
 loop { match pc {
 0x60c2eca => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2efd; continue; }
  pc = 0x60c2ed2; continue;
 }
 0x60c2ed2 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2eca; continue; }
  pc = 0x60c2efd; continue;
 }
 0x60c2efd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2efe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2efe); }
 let mut pc: u32 = 0x60c2efe;
 loop { match pc {
 0x60c2efe => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2f19; continue; }
  pc = 0x60c2f06; continue;
 }
 0x60c2f06 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2efe; continue; }
  pc = 0x60c2f19; continue;
 }
 0x60c2f19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2f1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2f1a); }
 let mut pc: u32 = 0x60c2f1a;
 loop { match pc {
 0x60c2f1a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2f38; continue; }
  pc = 0x60c2f22; continue;
 }
 0x60c2f22 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2f1a; continue; }
  pc = 0x60c2f38; continue;
 }
 0x60c2f38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2f39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2f39); }
 let mut pc: u32 = 0x60c2f39;
 loop { match pc {
 0x60c2f39 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2f5f; continue; }
  pc = 0x60c2f41; continue;
 }
 0x60c2f41 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2f39; continue; }
  pc = 0x60c2f5f; continue;
 }
 0x60c2f5f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2f60); }
 let mut pc: u32 = 0x60c2f60;
 loop { match pc {
 0x60c2f60 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2f86; continue; }
  pc = 0x60c2f68; continue;
 }
 0x60c2f68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2f60; continue; }
  pc = 0x60c2f86; continue;
 }
 0x60c2f86 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2f87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2f87); }
 let mut pc: u32 = 0x60c2f87;
 loop { match pc {
 0x60c2f87 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2fa5; continue; }
  pc = 0x60c2f8f; continue;
 }
 0x60c2f8f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2f87; continue; }
  pc = 0x60c2fa5; continue;
 }
 0x60c2fa5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2fa6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2fa6); }
 let mut pc: u32 = 0x60c2fa6;
 loop { match pc {
 0x60c2fa6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2fc7; continue; }
  pc = 0x60c2fae; continue;
 }
 0x60c2fae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2fa6; continue; }
  pc = 0x60c2fc7; continue;
 }
 0x60c2fc7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2fc8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2fc8); }
 let mut pc: u32 = 0x60c2fc8;
 loop { match pc {
 0x60c2fc8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c2ff0; continue; }
  pc = 0x60c2fd0; continue;
 }
 0x60c2fd0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2fc8; continue; }
  pc = 0x60c2ff0; continue;
 }
 0x60c2ff0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c2ff1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c2ff1); }
 let mut pc: u32 = 0x60c2ff1;
 loop { match pc {
 0x60c2ff1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3019; continue; }
  pc = 0x60c2ff9; continue;
 }
 0x60c2ff9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c2ff1; continue; }
  pc = 0x60c3019; continue;
 }
 0x60c3019 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c301a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c301a); }
 let mut pc: u32 = 0x60c301a;
 loop { match pc {
 0x60c301a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3045; continue; }
  pc = 0x60c3022; continue;
 }
 0x60c3022 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c301a; continue; }
  pc = 0x60c3045; continue;
 }
 0x60c3045 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3046(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3046); }
 let mut pc: u32 = 0x60c3046;
 loop { match pc {
 0x60c3046 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3074; continue; }
  pc = 0x60c304e; continue;
 }
 0x60c304e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3046; continue; }
  pc = 0x60c3074; continue;
 }
 0x60c3074 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3075(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3075); }
 let mut pc: u32 = 0x60c3075;
 loop { match pc {
 0x60c3075 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c30b4; continue; }
  pc = 0x60c307d; continue;
 }
 0x60c307d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3075; continue; }
  pc = 0x60c30b4; continue;
 }
 0x60c30b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c30b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c30b5); }
 let mut pc: u32 = 0x60c30b5;
 loop { match pc {
 0x60c30b5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c30f4; continue; }
  pc = 0x60c30bd; continue;
 }
 0x60c30bd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c30b5; continue; }
  pc = 0x60c30f4; continue;
 }
 0x60c30f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c30f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c30f5); }
 let mut pc: u32 = 0x60c30f5;
 loop { match pc {
 0x60c30f5 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3126; continue; }
  pc = 0x60c30fd; continue;
 }
 0x60c30fd => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c30f5; continue; }
  pc = 0x60c3126; continue;
 }
 0x60c3126 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3127); }
 let mut pc: u32 = 0x60c3127;
 loop { match pc {
 0x60c3127 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c315b; continue; }
  pc = 0x60c312f; continue;
 }
 0x60c312f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3127; continue; }
  pc = 0x60c315b; continue;
 }
 0x60c315b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c315c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c315c); }
 let mut pc: u32 = 0x60c315c;
 loop { match pc {
 0x60c315c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c319d; continue; }
  pc = 0x60c3164; continue;
 }
 0x60c3164 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c315c; continue; }
  pc = 0x60c319d; continue;
 }
 0x60c319d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c319e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c319e); }
 let mut pc: u32 = 0x60c319e;
 loop { match pc {
 0x60c319e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c31df; continue; }
  pc = 0x60c31a6; continue;
 }
 0x60c31a6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c319e; continue; }
  pc = 0x60c31df; continue;
 }
 0x60c31df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c31e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c31e0); }
 let mut pc: u32 = 0x60c31e0;
 loop { match pc {
 0x60c31e0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3206; continue; }
  pc = 0x60c31e8; continue;
 }
 0x60c31e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c31e0; continue; }
  pc = 0x60c3206; continue;
 }
 0x60c3206 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3207(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3207); }
 let mut pc: u32 = 0x60c3207;
 loop { match pc {
 0x60c3207 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3230; continue; }
  pc = 0x60c320f; continue;
 }
 0x60c320f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3207; continue; }
  pc = 0x60c3230; continue;
 }
 0x60c3230 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3231(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3231); }
 let mut pc: u32 = 0x60c3231;
 loop { match pc {
 0x60c3231 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3265; continue; }
  pc = 0x60c3239; continue;
 }
 0x60c3239 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3231; continue; }
  pc = 0x60c3265; continue;
 }
 0x60c3265 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3266(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3266); }
 let mut pc: u32 = 0x60c3266;
 loop { match pc {
 0x60c3266 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c329a; continue; }
  pc = 0x60c326e; continue;
 }
 0x60c326e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3266; continue; }
  pc = 0x60c329a; continue;
 }
 0x60c329a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c329b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c329b); }
 let mut pc: u32 = 0x60c329b;
 loop { match pc {
 0x60c329b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c32c7; continue; }
  pc = 0x60c32a3; continue;
 }
 0x60c32a3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c329b; continue; }
  pc = 0x60c32c7; continue;
 }
 0x60c32c7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c32c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c32c8); }
 let mut pc: u32 = 0x60c32c8;
 loop { match pc {
 0x60c32c8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c32f7; continue; }
  pc = 0x60c32d0; continue;
 }
 0x60c32d0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c32c8; continue; }
  pc = 0x60c32f7; continue;
 }
 0x60c32f7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c32f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c32f8); }
 let mut pc: u32 = 0x60c32f8;
 loop { match pc {
 0x60c32f8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c332e; continue; }
  pc = 0x60c3300; continue;
 }
 0x60c3300 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c32f8; continue; }
  pc = 0x60c332e; continue;
 }
 0x60c332e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c332f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c332f); }
 let mut pc: u32 = 0x60c332f;
 loop { match pc {
 0x60c332f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3365; continue; }
  pc = 0x60c3337; continue;
 }
 0x60c3337 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c332f; continue; }
  pc = 0x60c3365; continue;
 }
 0x60c3365 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3366(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3366); }
 let mut pc: u32 = 0x60c3366;
 loop { match pc {
 0x60c3366 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3381; continue; }
  pc = 0x60c336e; continue;
 }
 0x60c336e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3366; continue; }
  pc = 0x60c3381; continue;
 }
 0x60c3381 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3382(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3382); }
 let mut pc: u32 = 0x60c3382;
 loop { match pc {
 0x60c3382 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c33a7; continue; }
  pc = 0x60c338a; continue;
 }
 0x60c338a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3382; continue; }
  pc = 0x60c33a7; continue;
 }
 0x60c33a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c33a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c33a8); }
 let mut pc: u32 = 0x60c33a8;
 loop { match pc {
 0x60c33a8 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c33d1; continue; }
  pc = 0x60c33b0; continue;
 }
 0x60c33b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c33a8; continue; }
  pc = 0x60c33d1; continue;
 }
 0x60c33d1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c33d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c33d2); }
 let mut pc: u32 = 0x60c33d2;
 loop { match pc {
 0x60c33d2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c33fc; continue; }
  pc = 0x60c33da; continue;
 }
 0x60c33da => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c33d2; continue; }
  pc = 0x60c33fc; continue;
 }
 0x60c33fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c33fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c33fd); }
 let mut pc: u32 = 0x60c33fd;
 loop { match pc {
 0x60c33fd => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3426; continue; }
  pc = 0x60c3405; continue;
 }
 0x60c3405 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c33fd; continue; }
  pc = 0x60c3426; continue;
 }
 0x60c3426 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3427(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3427); }
 let mut pc: u32 = 0x60c3427;
 loop { match pc {
 0x60c3427 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3451; continue; }
  pc = 0x60c342f; continue;
 }
 0x60c342f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3427; continue; }
  pc = 0x60c3451; continue;
 }
 0x60c3451 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3452); }
 let mut pc: u32 = 0x60c3452;
 loop { match pc {
 0x60c3452 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c347b; continue; }
  pc = 0x60c345a; continue;
 }
 0x60c345a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3452; continue; }
  pc = 0x60c347b; continue;
 }
 0x60c347b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c347c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c347c); }
 let mut pc: u32 = 0x60c347c;
 loop { match pc {
 0x60c347c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c34b5; continue; }
  pc = 0x60c3484; continue;
 }
 0x60c3484 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c347c; continue; }
  pc = 0x60c34b5; continue;
 }
 0x60c34b5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c34b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c34b6); }
 let mut pc: u32 = 0x60c34b6;
 loop { match pc {
 0x60c34b6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c34f8; continue; }
  pc = 0x60c34be; continue;
 }
 0x60c34be => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c34b6; continue; }
  pc = 0x60c34f8; continue;
 }
 0x60c34f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c34f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c34f9); }
 let mut pc: u32 = 0x60c34f9;
 loop { match pc {
 0x60c34f9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c353c; continue; }
  pc = 0x60c3501; continue;
 }
 0x60c3501 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c34f9; continue; }
  pc = 0x60c353c; continue;
 }
 0x60c353c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c353d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c353d); }
 let mut pc: u32 = 0x60c353d;
 loop { match pc {
 0x60c353d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c357f; continue; }
  pc = 0x60c3545; continue;
 }
 0x60c3545 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c353d; continue; }
  pc = 0x60c357f; continue;
 }
 0x60c357f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3580); }
 let mut pc: u32 = 0x60c3580;
 loop { match pc {
 0x60c3580 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c35c3; continue; }
  pc = 0x60c3588; continue;
 }
 0x60c3588 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3580; continue; }
  pc = 0x60c35c3; continue;
 }
 0x60c35c3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c35c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c35c4); }
 let mut pc: u32 = 0x60c35c4;
 loop { match pc {
 0x60c35c4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c35e8; continue; }
  pc = 0x60c35cc; continue;
 }
 0x60c35cc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c35c4; continue; }
  pc = 0x60c35e8; continue;
 }
 0x60c35e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c35e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c35e9); }
 let mut pc: u32 = 0x60c35e9;
 loop { match pc {
 0x60c35e9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c361d; continue; }
  pc = 0x60c35f1; continue;
 }
 0x60c35f1 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c35e9; continue; }
  pc = 0x60c361d; continue;
 }
 0x60c361d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c361e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c361e); }
 let mut pc: u32 = 0x60c361e;
 loop { match pc {
 0x60c361e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3655; continue; }
  pc = 0x60c3626; continue;
 }
 0x60c3626 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c361e; continue; }
  pc = 0x60c3655; continue;
 }
 0x60c3655 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3656); }
 let mut pc: u32 = 0x60c3656;
 loop { match pc {
 0x60c3656 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c368e; continue; }
  pc = 0x60c365e; continue;
 }
 0x60c365e => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3656; continue; }
  pc = 0x60c368e; continue;
 }
 0x60c368e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c368f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c368f); }
 let mut pc: u32 = 0x60c368f;
 loop { match pc {
 0x60c368f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c36c6; continue; }
  pc = 0x60c3697; continue;
 }
 0x60c3697 => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c368f; continue; }
  pc = 0x60c36c6; continue;
 }
 0x60c36c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c36c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c36c7); }
 let mut pc: u32 = 0x60c36c7;
 loop { match pc {
 0x60c36c7 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c36ff; continue; }
  pc = 0x60c36cf; continue;
 }
 0x60c36cf => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c36c7; continue; }
  pc = 0x60c36ff; continue;
 }
 0x60c36ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3700); }
 let mut pc: u32 = 0x60c3700;
 loop { match pc {
 0x60c3700 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3726; continue; }
  pc = 0x60c3708; continue;
 }
 0x60c3708 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3700; continue; }
  pc = 0x60c3726; continue;
 }
 0x60c3726 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3727(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3727); }
 let mut pc: u32 = 0x60c3727;
 loop { match pc {
 0x60c3727 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3757; continue; }
  pc = 0x60c372f; continue;
 }
 0x60c372f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x617ca88u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3727; continue; }
  pc = 0x60c3757; continue;
 }
 0x60c3757 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3758(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3758); }
 let mut pc: u32 = 0x60c3758;
 loop { match pc {
 0x60c3758 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c378f; continue; }
  pc = 0x60c3760; continue;
 }
 0x60c3760 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3758; continue; }
  pc = 0x60c378f; continue;
 }
 0x60c378f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3790); }
 let mut pc: u32 = 0x60c3790;
 loop { match pc {
 0x60c3790 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c37c8; continue; }
  pc = 0x60c3798; continue;
 }
 0x60c3798 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3790; continue; }
  pc = 0x60c37c8; continue;
 }
 0x60c37c8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c37c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c37c9); }
 let mut pc: u32 = 0x60c37c9;
 loop { match pc {
 0x60c37c9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3800; continue; }
  pc = 0x60c37d1; continue;
 }
 0x60c37d1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c37c9; continue; }
  pc = 0x60c3800; continue;
 }
 0x60c3800 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3801(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3801); }
 let mut pc: u32 = 0x60c3801;
 loop { match pc {
 0x60c3801 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3839; continue; }
  pc = 0x60c3809; continue;
 }
 0x60c3809 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3801; continue; }
  pc = 0x60c3839; continue;
 }
 0x60c3839 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c383a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c383a); }
 let mut pc: u32 = 0x60c383a;
 loop { match pc {
 0x60c383a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3871; continue; }
  pc = 0x60c3842; continue;
 }
 0x60c3842 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c383a; continue; }
  pc = 0x60c3871; continue;
 }
 0x60c3871 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3872(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3872); }
 let mut pc: u32 = 0x60c3872;
 loop { match pc {
 0x60c3872 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c38b9; continue; }
  pc = 0x60c387a; continue;
 }
 0x60c387a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3872; continue; }
  pc = 0x60c38b9; continue;
 }
 0x60c38b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c38ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c38ba); }
 let mut pc: u32 = 0x60c38ba;
 loop { match pc {
 0x60c38ba => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c390a; continue; }
  pc = 0x60c38c2; continue;
 }
 0x60c38c2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c38ba; continue; }
  pc = 0x60c390a; continue;
 }
 0x60c390a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c390b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c390b); }
 let mut pc: u32 = 0x60c390b;
 loop { match pc {
 0x60c390b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c395c; continue; }
  pc = 0x60c3913; continue;
 }
 0x60c3913 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c390b; continue; }
  pc = 0x60c395c; continue;
 }
 0x60c395c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c395d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c395d); }
 let mut pc: u32 = 0x60c395d;
 loop { match pc {
 0x60c395d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c39ad; continue; }
  pc = 0x60c3965; continue;
 }
 0x60c3965 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c395d; continue; }
  pc = 0x60c39ad; continue;
 }
 0x60c39ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c39ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c39ae); }
 let mut pc: u32 = 0x60c39ae;
 loop { match pc {
 0x60c39ae => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c39ff; continue; }
  pc = 0x60c39b6; continue;
 }
 0x60c39b6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c39ae; continue; }
  pc = 0x60c39ff; continue;
 }
 0x60c39ff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3a00); }
 let mut pc: u32 = 0x60c3a00;
 loop { match pc {
 0x60c3a00 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3a32; continue; }
  pc = 0x60c3a08; continue;
 }
 0x60c3a08 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3a00; continue; }
  pc = 0x60c3a32; continue;
 }
 0x60c3a32 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3a33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3a33); }
 let mut pc: u32 = 0x60c3a33;
 loop { match pc {
 0x60c3a33 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3a75; continue; }
  pc = 0x60c3a3b; continue;
 }
 0x60c3a3b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3a33; continue; }
  pc = 0x60c3a75; continue;
 }
 0x60c3a75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3a76); }
 let mut pc: u32 = 0x60c3a76;
 loop { match pc {
 0x60c3a76 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3abb; continue; }
  pc = 0x60c3a7e; continue;
 }
 0x60c3a7e => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3a76; continue; }
  pc = 0x60c3abb; continue;
 }
 0x60c3abb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3abc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3abc); }
 let mut pc: u32 = 0x60c3abc;
 loop { match pc {
 0x60c3abc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3b02; continue; }
  pc = 0x60c3ac4; continue;
 }
 0x60c3ac4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3abc; continue; }
  pc = 0x60c3b02; continue;
 }
 0x60c3b02 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3b03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3b03); }
 let mut pc: u32 = 0x60c3b03;
 loop { match pc {
 0x60c3b03 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3b48; continue; }
  pc = 0x60c3b0b; continue;
 }
 0x60c3b0b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3b03; continue; }
  pc = 0x60c3b48; continue;
 }
 0x60c3b48 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3b49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3b49); }
 let mut pc: u32 = 0x60c3b49;
 loop { match pc {
 0x60c3b49 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x617ca78u32)); }
  if !cpu.cf { pc = 0x60c3b8f; continue; }
  pc = 0x60c3b51; continue;
 }
 0x60c3b51 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x617ca84u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x617ca88u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x617ca80u32)); }
  if cpu.cf { cpu.poll(); pc = 0x60c3b49; continue; }
  pc = 0x60c3b8f; continue;
 }
 0x60c3b8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
