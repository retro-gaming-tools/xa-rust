#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_030e04e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e04e6); }
 let mut pc: u32 = 0x30e04e6;
 loop { match pc {
 0x30e04e6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e04e9, "Pxor");
  cpu.unimplemented(0x30e04ec, "Pxor");
  cpu.unimplemented(0x30e04ef, "Movd");
  cpu.unimplemented(0x30e04f3, "Punpckldq");
  cpu.unimplemented(0x30e04f7, "Movd");
  cpu.unimplemented(0x30e04fb, "Movq");
  cpu.unimplemented(0x30e04fe, "Pand");
  cpu.unimplemented(0x30e0505, "Psrld");
  cpu.unimplemented(0x30e0509, "Movq");
  cpu.unimplemented(0x30e050c, "Punpckhwd");
  cpu.unimplemented(0x30e050f, "Punpckldq");
  cpu.unimplemented(0x30e0512, "Movq");
  cpu.unimplemented(0x30e0515, "Pxor");
  cpu.unimplemented(0x30e051c, "Pxor");
  cpu.unimplemented(0x30e0523, "Movq");
  cpu.unimplemented(0x30e052a, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e053a; continue;
 }
 0x30e053a => {
  cpu.unimplemented(0x30e053a, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e0540, "Movq");
  cpu.unimplemented(0x30e0544, "Movq");
  cpu.unimplemented(0x30e0547, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e054f, "Movq");
  cpu.unimplemented(0x30e0554, "Punpcklbw");
  cpu.unimplemented(0x30e0558, "Punpcklbw");
  cpu.unimplemented(0x30e055b, "Por");
  cpu.unimplemented(0x30e055e, "Movq");
  cpu.unimplemented(0x30e0561, "Movq");
  cpu.unimplemented(0x30e0564, "Pand");
  cpu.unimplemented(0x30e056b, "Psrlq");
  cpu.unimplemented(0x30e056f, "Psrld");
  cpu.unimplemented(0x30e0573, "Por");
  cpu.unimplemented(0x30e0576, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e0676; continue; }
  pc = 0x30e0582; continue;
 }
 0x30e0582 => {
  cpu.unimplemented(0x30e0582, "Punpcklwd");
  cpu.unimplemented(0x30e0585, "Movq");
  cpu.unimplemented(0x30e058c, "Punpckldq");
  cpu.unimplemented(0x30e058f, "Pmulhw");
  cpu.unimplemented(0x30e0592, "Movq");
  cpu.unimplemented(0x30e0599, "Pxor");
  cpu.unimplemented(0x30e05a0, "Pmulhw");
  cpu.unimplemented(0x30e05a3, "Movq");
  cpu.unimplemented(0x30e05aa, "Movq");
  cpu.unimplemented(0x30e05b1, "Punpckldq");
  cpu.unimplemented(0x30e05b4, "Pxor");
  cpu.unimplemented(0x30e05b7, "Movq");
  cpu.unimplemented(0x30e05ba, "Punpcklbw");
  cpu.unimplemented(0x30e05bd, "Movq");
  cpu.unimplemented(0x30e05c0, "Punpcklbw");
  cpu.unimplemented(0x30e05c3, "Punpckhwd");
  cpu.unimplemented(0x30e05c6, "Punpcklbw");
  cpu.unimplemented(0x30e05c9, "Pmaddwd");
  cpu.unimplemented(0x30e05d0, "Pmaddwd");
  cpu.unimplemented(0x30e05d3, "Pmaddwd");
  cpu.unimplemented(0x30e05da, "Punpckldq");
  cpu.unimplemented(0x30e05dd, "Paddd");
  cpu.unimplemented(0x30e05e0, "Paddd");
  cpu.unimplemented(0x30e05e3, "Psrlq");
  cpu.unimplemented(0x30e05e7, "Movq");
  cpu.unimplemented(0x30e05ea, "Pand");
  cpu.unimplemented(0x30e05f1, "Psrlq");
  cpu.unimplemented(0x30e05f5, "Pand");
  cpu.unimplemented(0x30e05fc, "Por");
  cpu.unimplemented(0x30e05ff, "Psrlq");
  cpu.unimplemented(0x30e0603, "Pand");
  cpu.unimplemented(0x30e060a, "Por");
  cpu.unimplemented(0x30e060d, "Movd");
  cpu.unimplemented(0x30e0610, "Movq");
  cpu.unimplemented(0x30e0613, "Psllw");
  cpu.unimplemented(0x30e0617, "Movq");
  cpu.unimplemented(0x30e061a, "Pand");
  cpu.unimplemented(0x30e0621, "Psllw");
  cpu.unimplemented(0x30e0625, "Psrlw");
  cpu.unimplemented(0x30e0629, "Pand");
  cpu.unimplemented(0x30e0630, "Por");
  cpu.unimplemented(0x30e0633, "Pand");
  cpu.unimplemented(0x30e063a, "Movq");
  cpu.unimplemented(0x30e063d, "Punpcklwd");
  cpu.unimplemented(0x30e0640, "Paddusb");
  cpu.unimplemented(0x30e0643, "Movq");
  cpu.unimplemented(0x30e0646, "Psrld");
  cpu.unimplemented(0x30e064a, "Movq");
  cpu.unimplemented(0x30e064d, "Pand");
  cpu.unimplemented(0x30e0654, "Psrld");
  cpu.unimplemented(0x30e0658, "Pand");
  cpu.unimplemented(0x30e065f, "Psrld");
  cpu.unimplemented(0x30e0663, "Por");
  cpu.unimplemented(0x30e0666, "Pand");
  cpu.unimplemented(0x30e066d, "Por");
  cpu.unimplemented(0x30e0670, "Movd");
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30e0676; continue;
 }
 0x30e0676 => {
  cpu.unimplemented(0x30e0676, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e053a; continue; }
  pc = 0x30e0685; continue;
 }
 0x30e0685 => {
  cpu.unimplemented(0x30e0685, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e0689(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e0689); }
 let mut pc: u32 = 0x30e0689;
 loop { match pc {
 0x30e0689 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e068c, "Pxor");
  cpu.unimplemented(0x30e068f, "Pxor");
  cpu.unimplemented(0x30e0692, "Movd");
  cpu.unimplemented(0x30e0696, "Punpckldq");
  cpu.unimplemented(0x30e069a, "Movd");
  cpu.unimplemented(0x30e069e, "Movq");
  cpu.unimplemented(0x30e06a1, "Pand");
  cpu.unimplemented(0x30e06a8, "Psrld");
  cpu.unimplemented(0x30e06ac, "Movq");
  cpu.unimplemented(0x30e06af, "Punpckhwd");
  cpu.unimplemented(0x30e06b2, "Punpckldq");
  cpu.unimplemented(0x30e06b5, "Movq");
  cpu.unimplemented(0x30e06b8, "Pxor");
  cpu.unimplemented(0x30e06bf, "Pxor");
  cpu.unimplemented(0x30e06c6, "Movq");
  cpu.unimplemented(0x30e06cd, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e06dd; continue;
 }
 0x30e06dd => {
  cpu.unimplemented(0x30e06dd, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e06e3, "Movq");
  cpu.unimplemented(0x30e06e7, "Movq");
  cpu.unimplemented(0x30e06ea, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e06f2, "Movq");
  cpu.unimplemented(0x30e06f7, "Punpcklbw");
  cpu.unimplemented(0x30e06fb, "Punpcklbw");
  cpu.unimplemented(0x30e06fe, "Por");
  cpu.unimplemented(0x30e0701, "Movq");
  cpu.unimplemented(0x30e0704, "Movq");
  cpu.unimplemented(0x30e0707, "Pand");
  cpu.unimplemented(0x30e070e, "Psrlq");
  cpu.unimplemented(0x30e0712, "Psrld");
  cpu.unimplemented(0x30e0716, "Por");
  cpu.unimplemented(0x30e0719, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e0819; continue; }
  pc = 0x30e0725; continue;
 }
 0x30e0725 => {
  cpu.unimplemented(0x30e0725, "Punpcklwd");
  cpu.unimplemented(0x30e0728, "Movq");
  cpu.unimplemented(0x30e072f, "Punpckldq");
  cpu.unimplemented(0x30e0732, "Pmulhw");
  cpu.unimplemented(0x30e0735, "Movq");
  cpu.unimplemented(0x30e073c, "Pxor");
  cpu.unimplemented(0x30e0743, "Pmulhw");
  cpu.unimplemented(0x30e0746, "Movq");
  cpu.unimplemented(0x30e074d, "Movq");
  cpu.unimplemented(0x30e0754, "Punpckldq");
  cpu.unimplemented(0x30e0757, "Pxor");
  cpu.unimplemented(0x30e075a, "Movq");
  cpu.unimplemented(0x30e075d, "Punpcklbw");
  cpu.unimplemented(0x30e0760, "Movq");
  cpu.unimplemented(0x30e0763, "Punpcklbw");
  cpu.unimplemented(0x30e0766, "Punpckhwd");
  cpu.unimplemented(0x30e0769, "Punpcklbw");
  cpu.unimplemented(0x30e076c, "Pmaddwd");
  cpu.unimplemented(0x30e0773, "Pmaddwd");
  cpu.unimplemented(0x30e0776, "Pmaddwd");
  cpu.unimplemented(0x30e077d, "Punpckldq");
  cpu.unimplemented(0x30e0780, "Paddd");
  cpu.unimplemented(0x30e0783, "Paddd");
  cpu.unimplemented(0x30e0786, "Psrlq");
  cpu.unimplemented(0x30e078a, "Movq");
  cpu.unimplemented(0x30e078d, "Pand");
  cpu.unimplemented(0x30e0794, "Psrlq");
  cpu.unimplemented(0x30e0798, "Pand");
  cpu.unimplemented(0x30e079f, "Por");
  cpu.unimplemented(0x30e07a2, "Psrlq");
  cpu.unimplemented(0x30e07a6, "Pand");
  cpu.unimplemented(0x30e07ad, "Por");
  cpu.unimplemented(0x30e07b0, "Movd");
  cpu.unimplemented(0x30e07b3, "Movq");
  cpu.unimplemented(0x30e07b6, "Psllw");
  cpu.unimplemented(0x30e07ba, "Movq");
  cpu.unimplemented(0x30e07bd, "Pand");
  cpu.unimplemented(0x30e07c4, "Psllw");
  cpu.unimplemented(0x30e07c8, "Psrlw");
  cpu.unimplemented(0x30e07cc, "Pand");
  cpu.unimplemented(0x30e07d3, "Por");
  cpu.unimplemented(0x30e07d6, "Pand");
  cpu.unimplemented(0x30e07dd, "Movq");
  cpu.unimplemented(0x30e07e0, "Punpcklwd");
  cpu.unimplemented(0x30e07e3, "Psubusb");
  cpu.unimplemented(0x30e07e6, "Movq");
  cpu.unimplemented(0x30e07e9, "Psrld");
  cpu.unimplemented(0x30e07ed, "Movq");
  cpu.unimplemented(0x30e07f0, "Pand");
  cpu.unimplemented(0x30e07f7, "Psrld");
  cpu.unimplemented(0x30e07fb, "Pand");
  cpu.unimplemented(0x30e0802, "Psrld");
  cpu.unimplemented(0x30e0806, "Por");
  cpu.unimplemented(0x30e0809, "Pand");
  cpu.unimplemented(0x30e0810, "Por");
  cpu.unimplemented(0x30e0813, "Movd");
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30e0819; continue;
 }
 0x30e0819 => {
  cpu.unimplemented(0x30e0819, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e06dd; continue; }
  pc = 0x30e0828; continue;
 }
 0x30e0828 => {
  cpu.unimplemented(0x30e0828, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e082c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e082c); }
 let mut pc: u32 = 0x30e082c;
 loop { match pc {
 0x30e082c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e082f, "Pxor");
  cpu.unimplemented(0x30e0832, "Pxor");
  cpu.unimplemented(0x30e0835, "Movd");
  cpu.unimplemented(0x30e0839, "Punpckldq");
  cpu.unimplemented(0x30e083d, "Movd");
  cpu.unimplemented(0x30e0841, "Movq");
  cpu.unimplemented(0x30e0844, "Pand");
  cpu.unimplemented(0x30e084b, "Psrld");
  cpu.unimplemented(0x30e084f, "Movq");
  cpu.unimplemented(0x30e0852, "Punpckhwd");
  cpu.unimplemented(0x30e0855, "Punpckldq");
  cpu.unimplemented(0x30e0858, "Movq");
  cpu.unimplemented(0x30e085b, "Pxor");
  cpu.unimplemented(0x30e0862, "Pxor");
  cpu.unimplemented(0x30e0869, "Movq");
  cpu.unimplemented(0x30e0870, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e0880; continue;
 }
 0x30e0880 => {
  cpu.unimplemented(0x30e0880, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e0886, "Movq");
  cpu.unimplemented(0x30e088a, "Movq");
  cpu.unimplemented(0x30e088d, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e0895, "Movq");
  cpu.unimplemented(0x30e089a, "Punpcklbw");
  cpu.unimplemented(0x30e089e, "Punpcklbw");
  cpu.unimplemented(0x30e08a1, "Por");
  cpu.unimplemented(0x30e08a4, "Movq");
  cpu.unimplemented(0x30e08a7, "Movq");
  cpu.unimplemented(0x30e08aa, "Pand");
  cpu.unimplemented(0x30e08b1, "Psrlq");
  cpu.unimplemented(0x30e08b5, "Psrld");
  cpu.unimplemented(0x30e08b9, "Por");
  cpu.unimplemented(0x30e08bc, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e0986; continue; }
  pc = 0x30e08c8; continue;
 }
 0x30e08c8 => {
  cpu.unimplemented(0x30e08c8, "Punpcklwd");
  cpu.unimplemented(0x30e08cb, "Movq");
  cpu.unimplemented(0x30e08d2, "Punpckldq");
  cpu.unimplemented(0x30e08d5, "Pmulhw");
  cpu.unimplemented(0x30e08d8, "Movq");
  cpu.unimplemented(0x30e08df, "Pxor");
  cpu.unimplemented(0x30e08e6, "Pmulhw");
  cpu.unimplemented(0x30e08e9, "Movq");
  cpu.unimplemented(0x30e08f0, "Movq");
  cpu.unimplemented(0x30e08f7, "Punpckldq");
  cpu.unimplemented(0x30e08fa, "Pxor");
  cpu.unimplemented(0x30e08fd, "Movq");
  cpu.unimplemented(0x30e0900, "Punpcklbw");
  cpu.unimplemented(0x30e0903, "Movq");
  cpu.unimplemented(0x30e0906, "Punpcklbw");
  cpu.unimplemented(0x30e0909, "Punpckhwd");
  cpu.unimplemented(0x30e090c, "Punpcklbw");
  cpu.unimplemented(0x30e090f, "Pmaddwd");
  cpu.unimplemented(0x30e0916, "Pmaddwd");
  cpu.unimplemented(0x30e0919, "Pmaddwd");
  cpu.unimplemented(0x30e0920, "Punpckldq");
  cpu.unimplemented(0x30e0923, "Paddd");
  cpu.unimplemented(0x30e0926, "Paddd");
  cpu.unimplemented(0x30e0929, "Psrlq");
  cpu.unimplemented(0x30e092d, "Movq");
  cpu.unimplemented(0x30e0930, "Pand");
  cpu.unimplemented(0x30e0937, "Psrlq");
  cpu.unimplemented(0x30e093b, "Pand");
  cpu.unimplemented(0x30e0942, "Por");
  cpu.unimplemented(0x30e0945, "Psrlq");
  cpu.unimplemented(0x30e0949, "Pand");
  cpu.unimplemented(0x30e0950, "Por");
  cpu.unimplemented(0x30e0953, "Movq");
  cpu.unimplemented(0x30e0956, "Psrld");
  cpu.unimplemented(0x30e095a, "Movq");
  cpu.unimplemented(0x30e095d, "Pand");
  cpu.unimplemented(0x30e0964, "Psrld");
  cpu.unimplemented(0x30e0968, "Pand");
  cpu.unimplemented(0x30e096f, "Psrld");
  cpu.unimplemented(0x30e0973, "Por");
  cpu.unimplemented(0x30e0976, "Pand");
  cpu.unimplemented(0x30e097d, "Por");
  cpu.unimplemented(0x30e0980, "Movd");
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30e0986; continue;
 }
 0x30e0986 => {
  cpu.unimplemented(0x30e0986, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e0880; continue; }
  pc = 0x30e0995; continue;
 }
 0x30e0995 => {
  cpu.unimplemented(0x30e0995, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e0999(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e0999); }
 let mut pc: u32 = 0x30e0999;
 loop { match pc {
 0x30e0999 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x30e09a5; continue;
 }
 0x30e09a5 => {
  cpu.unimplemented(0x30e09a5, "Movq");
  cpu.unimplemented(0x30e09a8, "Movq");
  cpu.unimplemented(0x30e09ab, "Psllw");
  cpu.unimplemented(0x30e09af, "Movq");
  cpu.unimplemented(0x30e09b2, "Pand");
  cpu.unimplemented(0x30e09b9, "Psllw");
  cpu.unimplemented(0x30e09bd, "Psrlw");
  cpu.unimplemented(0x30e09c1, "Pand");
  cpu.unimplemented(0x30e09c8, "Por");
  cpu.unimplemented(0x30e09cb, "Pand");
  cpu.unimplemented(0x30e09d2, "Movq");
  cpu.unimplemented(0x30e09d5, "Punpcklwd");
  cpu.unimplemented(0x30e09d8, "Punpckhwd");
  cpu.unimplemented(0x30e09db, "Movq");
  cpu.unimplemented(0x30e09de, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x4u32); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30e09a5; continue; }
  pc = 0x30e09ed; continue;
 }
 0x30e09ed => {
  cpu.unimplemented(0x30e09ed, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030f0000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30f0000); }
 let mut pc: u32 = 0x30f0000;
 loop { match pc {
 0x30f0000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f002cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0030u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0038u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0034u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f003cu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0040u32, v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030f0035(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30f0035); }
 let mut pc: u32 = 0x30f0035;
 loop { match pc {
 0x30f0035 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30f00acu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0020u32, v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0024u32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x7u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32f0028u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32f0044u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x30f006d; continue;
 }
 0x30f006d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.edi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x30f00aa; continue; }
  pc = 0x30f0073; continue;
 }
 0x30f0073 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.r32(0x32f0028u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32f0044u32)); cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x32f0024u32); cpu.push(v); }
  { let v = cpu.r32(0x32f0020u32); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x30f009c); f_030f00f0(cpu); if cpu.returned_to != 0x30f009c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32f0020u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x30f006d; continue;
 }
 0x30f00aa => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030f00ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30f00ac); } r_030f00ac(cpu, 0x30f00ac); }
pub fn f_030f00f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30f00f0); } r_030f00ac(cpu, 0x30f00f0); }
fn r_030f00ac(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30f00ac => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00b2, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00b6, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00ba, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00be, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00c2, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00c6, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00ca, "Das");
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x2fu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)))); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00d2, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00d6, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00da, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00de, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00e2, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00e6, "Das");
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  cpu.unimplemented(0x30f00ea, "Das");
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xeu32.wrapping_add(cpu.eax))); cpu.edx = v; }
  cpu.unimplemented(0x30f00ee, "Das");
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xffffff8bu32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32f002cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x32f0040u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f000cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0008u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0010u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32f0038u32)); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30f0158; continue; }
  pc = 0x30f0137; continue;
 }
 0x30f00f0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32f002cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x32f0040u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f000cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0008u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0010u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32f0038u32)); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30f0158; continue; }
  pc = 0x30f0137; continue;
 }
 0x30f0137 => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x32f000cu32), cpu.ebx); cpu.w32(0x32f000cu32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f02fb; continue; }
  pc = 0x30f0145; continue;
 }
 0x30f0145 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.r32(0x32f0040u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30f0158; continue;
 }
 0x30f0158 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32f003cu32)); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f0174; continue; }
  pc = 0x30f0166; continue;
 }
 0x30f0166 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x32f000cu32), cpu.ebx); cpu.w32(0x32f000cu32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f02fb; continue; }
  pc = 0x30f0172; continue;
 }
 0x30f0172 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30f0174; continue;
 }
 0x30f0174 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32f0034u32)); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f0191; continue; }
  pc = 0x30f0182; continue;
 }
 0x30f0182 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0x32f0008u32), cpu.ebx); cpu.w32(0x32f0008u32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f02fb; continue; }
  pc = 0x30f018f; continue;
 }
 0x30f018f => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30f0191; continue;
 }
 0x30f0191 => {
  { let v: u32 = 0x0u32; cpu.w32(0x32f0018u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32f001cu32, v); }
  { let v: u32 = cpu.r32(0x32f0030u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f01d8; continue; }
  pc = 0x30f01b0; continue;
 }
 0x30f01b0 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x32f0008u32), cpu.ebx); cpu.w32(0x32f0008u32, v); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30f02fb; continue; }
  pc = 0x30f01bc; continue;
 }
 0x30f01bc => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f0018u32, v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f001cu32, v); }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30f01d8; continue;
 }
 0x30f01d8 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30f029c; continue; }
  pc = 0x30f01e1; continue;
 }
 0x30f01e1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x7u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0014u32, v); }
  { let v: u32 = cpu.r32(0x32f0008u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0004u32, v); }
  { let v: u32 = cpu.ecx & 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f0000u32, v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x30f020c; continue;
 }
 0x30f020c => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32f0004u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30f0261; continue; }
  pc = 0x30f0219; continue;
 }
 0x30f0219 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f0223; continue; }
  pc = 0x30f0220; continue;
 }
 0x30f0220 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f0223; continue;
 }
 0x30f0223 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f022b; continue; }
  pc = 0x30f0227; continue;
 }
 0x30f0227 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f022b; continue;
 }
 0x30f022b => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f0233; continue; }
  pc = 0x30f022f; continue;
 }
 0x30f022f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f0233; continue;
 }
 0x30f0233 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f023b; continue; }
  pc = 0x30f0237; continue;
 }
 0x30f0237 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f023b; continue;
 }
 0x30f023b => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f0243; continue; }
  pc = 0x30f023f; continue;
 }
 0x30f023f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f0243; continue;
 }
 0x30f0243 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f024b; continue; }
  pc = 0x30f0247; continue;
 }
 0x30f0247 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x30f024b; continue;
 }
 0x30f024b => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f0253; continue; }
  pc = 0x30f024f; continue;
 }
 0x30f024f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30f0253; continue;
 }
 0x30f0253 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f025b; continue; }
  pc = 0x30f0257; continue;
 }
 0x30f0257 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30f025b; continue;
 }
 0x30f025b => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30f0219; continue; }
  pc = 0x30f0261; continue;
 }
 0x30f0261 => {
  { let v: u32 = cpu.r32(0x32f0000u32); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30f0281; continue; }
  pc = 0x30f026b; continue;
 }
 0x30f026b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32f0000u32); cpu.ecx = v; }
  pc = 0x30f0274; continue;
 }
 0x30f0274 => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f027b; continue; }
  pc = 0x30f0278; continue;
 }
 0x30f0278 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f027b; continue;
 }
 0x30f027b => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30f0274; continue; }
  pc = 0x30f0281; continue;
 }
 0x30f0281 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x32f0040u32)); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32f0014u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32f000cu32)); cpu.w32(0x32f000cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30f020c; continue; }
  pc = 0x30f029a; continue;
 }
 0x30f029a => {
  pc = 0x30f02fb; continue;
 }
 0x30f029c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x7u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f0014u32, v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  pc = 0x30f02ad; continue;
 }
 0x30f02ad => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x32f001cu32)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32f0018u32); cpu.ecx = v; }
  { let v: u8 = cpu.shl8((cpu.edx as u8), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32f0008u32); cpu.ecx = v; }
  pc = 0x30f02ce; continue;
 }
 0x30f02ce => {
  { let v: u8 = cpu.add8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.cf { pc = 0x30f02d5; continue; }
  pc = 0x30f02d2; continue;
 }
 0x30f02d2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30f02d5; continue;
 }
 0x30f02d5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30f02e3; continue; }
  pc = 0x30f02db; continue;
 }
 0x30f02db => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  pc = 0x30f02e3; continue;
 }
 0x30f02e3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30f02ce; continue; }
  pc = 0x30f02e6; continue;
 }
 0x30f02e6 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x32f0040u32)); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32f0014u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32f000cu32)); cpu.w32(0x32f000cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30f02ad; continue; }
  pc = 0x30f02fb; continue;
 }
 0x30f02fb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03100000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3100000); }
 let mut pc: u32 = 0x3100000;
 loop { match pc {
 0x3100000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32525dcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x32525d8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310018a; continue; }
  pc = 0x3100023; continue;
 }
 0x3100023 => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310006d; continue; }
  pc = 0x310002f; continue;
 }
 0x310002f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100064; continue; }
  pc = 0x3100053; continue;
 }
 0x3100053 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100064; continue;
 }
 0x3100064 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310006d; continue;
 }
 0x310006d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x31000a2; continue;
 }
 0x3100072 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31000a2; continue;
 }
 0x31000a2 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ecx = v; }
  pc = 0x31000ab; continue;
 }
 0x31000ab => {
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
  if !cpu.zf { cpu.poll(); pc = 0x31000ab; continue; }
  pc = 0x31000ce; continue;
 }
 0x31000ce => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x3100072; continue; }
  pc = 0x31000d8; continue;
 }
 0x31000d8 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x3100114; continue;
 }
 0x3100114 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x3100114; continue; }
  pc = 0x3100137; continue;
 }
 0x3100137 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100185; continue; }
  pc = 0x3100147; continue;
 }
 0x3100147 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310017c; continue; }
  pc = 0x310016b; continue;
 }
 0x310016b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310017c; continue;
 }
 0x310017c => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100185; continue;
 }
 0x3100185 => {
  pc = 0x310027a; continue;
 }
 0x310018a => {
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31001d4; continue; }
  pc = 0x3100196; continue;
 }
 0x3100196 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31001cb; continue; }
  pc = 0x31001ba; continue;
 }
 0x31001ba => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31001cb; continue;
 }
 0x31001cb => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31001d4; continue;
 }
 0x31001d4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x31001f7; continue;
 }
 0x31001d9 => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { cpu.interrupt(0x10); }
  pc = 0x31001f7; continue;
 }
 0x31001f7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ecx = v; }
  pc = 0x3100200; continue;
 }
 0x3100200 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x3100200; continue; }
  pc = 0x3100223; continue;
 }
 0x3100223 => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.sf { cpu.poll(); pc = 0x31001d9; continue; }
  pc = 0x310022d; continue;
 }
 0x310022d => {
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { cpu.interrupt(0x10); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.ecx = v; }
  pc = 0x3100257; continue;
 }
 0x3100257 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x3100257; continue; }
  pc = 0x310027a; continue;
 }
 0x310027a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310027c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310027c); }
 let mut pc: u32 = 0x310027c;
 loop { match pc {
 0x310027c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31002f1; continue; }
  pc = 0x31002b3; continue;
 }
 0x31002b3 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31002e8; continue; }
  pc = 0x31002d7; continue;
 }
 0x31002d7 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31002e8; continue;
 }
 0x31002e8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31002f1; continue;
 }
 0x31002f1 => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x310032f; continue;
 }
 0x310032f => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x3100350; continue; }
  pc = 0x3100337; continue;
 }
 0x3100337 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310032f; continue; }
  pc = 0x310034b; continue;
 }
 0x310034b => {
  pc = 0x3100474; continue;
 }
 0x3100350 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x31003fb; continue; }
  pc = 0x3100361; continue;
 }
 0x3100361 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31003c6; continue; }
  pc = 0x3100388; continue;
 }
 0x3100388 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31003bd; continue; }
  pc = 0x31003ac; continue;
 }
 0x31003ac => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31003bd; continue;
 }
 0x31003bd => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31003c6; continue;
 }
 0x31003c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310032f; continue; }
  pc = 0x31003f9; continue;
 }
 0x31003f9 => {
  pc = 0x3100474; continue;
 }
 0x31003fb => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310045d; continue; }
  pc = 0x310041f; continue;
 }
 0x310041f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100454; continue; }
  pc = 0x3100443; continue;
 }
 0x3100443 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100454; continue;
 }
 0x3100454 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310045d; continue;
 }
 0x310045d => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310032f; continue; }
  pc = 0x3100474; continue;
 }
 0x3100474 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03100477(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3100477); }
 let mut pc: u32 = 0x3100477;
 loop { match pc {
 0x3100477 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffecu32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31004ec; continue; }
  pc = 0x31004ae; continue;
 }
 0x31004ae => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31004e3; continue; }
  pc = 0x31004d2; continue;
 }
 0x31004d2 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31004e3; continue;
 }
 0x31004e3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31004ec; continue;
 }
 0x31004ec => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
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
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x310052a; continue;
 }
 0x310052a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x310054b; continue; }
  pc = 0x3100532; continue;
 }
 0x3100532 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310052a; continue; }
  pc = 0x3100546; continue;
 }
 0x3100546 => {
  pc = 0x3100671; continue;
 }
 0x310054b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x31005f8; continue; }
  pc = 0x310055c; continue;
 }
 0x310055c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx & 0x3u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31005c1; continue; }
  pc = 0x3100583; continue;
 }
 0x3100583 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31005b8; continue; }
  pc = 0x31005a7; continue;
 }
 0x31005a7 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31005b8; continue;
 }
 0x31005b8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31005c1; continue;
 }
 0x31005c1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1u32; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310052a; continue; }
  pc = 0x31005f6; continue;
 }
 0x31005f6 => {
  pc = 0x3100671; continue;
 }
 0x31005f8 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310065a; continue; }
  pc = 0x310061c; continue;
 }
 0x310061c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100651; continue; }
  pc = 0x3100640; continue;
 }
 0x3100640 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100651; continue;
 }
 0x3100651 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310065a; continue;
 }
 0x310065a => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310052a; continue; }
  pc = 0x3100671; continue;
 }
 0x3100671 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03100674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3100674); }
 let mut pc: u32 = 0x3100674;
 loop { match pc {
 0x3100674 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31006c4; continue; }
  pc = 0x3100686; continue;
 }
 0x3100686 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31006bb; continue; }
  pc = 0x31006aa; continue;
 }
 0x31006aa => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31006bb; continue;
 }
 0x31006bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31006c4; continue;
 }
 0x31006c4 => {
  { let v: u32 = cpu.edi; cpu.w32(0x32f1e80u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x31006e6; continue; }
  pc = 0x31006db; continue;
 }
 0x31006db => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x31006e6; continue;
 }
 0x31006e6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31006fd; continue;
 }
 0x31006ed => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31006fd; continue;
 }
 0x31006fd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31006ed; continue; }
  pc = 0x3100700; continue;
 }
 0x3100700 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31006e6; continue; }
  pc = 0x3100709; continue;
 }
 0x3100709 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100720; continue;
 }
 0x3100710 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100720; continue;
 }
 0x3100720 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100710; continue; }
  pc = 0x3100723; continue;
 }
 0x3100723 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100791; continue; }
  pc = 0x3100753; continue;
 }
 0x3100753 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100788; continue; }
  pc = 0x3100777; continue;
 }
 0x3100777 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100788; continue;
 }
 0x3100788 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100791; continue;
 }
 0x3100791 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x31007af; continue;
 }
 0x31007af => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31007c6; continue;
 }
 0x31007b6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31007c6; continue;
 }
 0x31007c6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31007b6; continue; }
  pc = 0x31007c9; continue;
 }
 0x31007c9 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31007af; continue; }
  pc = 0x31007d2; continue;
 }
 0x31007d2 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x31007e9; continue;
 }
 0x31007d9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31007e9; continue;
 }
 0x31007e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31007d9; continue; }
  pc = 0x31007ec; continue;
 }
 0x31007ec => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100841; continue; }
  pc = 0x3100803; continue;
 }
 0x3100803 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100838; continue; }
  pc = 0x3100827; continue;
 }
 0x3100827 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100838; continue;
 }
 0x3100838 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100841; continue;
 }
 0x3100841 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x310085e; continue;
 }
 0x310084e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310085e; continue;
 }
 0x310085e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310084e; continue; }
  pc = 0x3100861; continue;
 }
 0x3100861 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x310087f; continue;
 }
 0x310087f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100896; continue;
 }
 0x3100886 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100896; continue;
 }
 0x3100896 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100886; continue; }
  pc = 0x3100899; continue;
 }
 0x3100899 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310087f; continue; }
  pc = 0x31008a2; continue;
 }
 0x31008a2 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31008b9; continue;
 }
 0x31008a9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31008b9; continue;
 }
 0x31008b9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31008a9; continue; }
  pc = 0x31008bc; continue;
 }
 0x31008bc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310092a; continue; }
  pc = 0x31008ec; continue;
 }
 0x31008ec => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100921; continue; }
  pc = 0x3100910; continue;
 }
 0x3100910 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100921; continue;
 }
 0x3100921 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310092a; continue;
 }
 0x310092a => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3100948; continue;
 }
 0x3100948 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x310095f; continue;
 }
 0x310094f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310095f; continue;
 }
 0x310095f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310094f; continue; }
  pc = 0x3100962; continue;
 }
 0x3100962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100948; continue; }
  pc = 0x310096b; continue;
 }
 0x310096b => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x3100982; continue;
 }
 0x3100972 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100982; continue;
 }
 0x3100982 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100972; continue; }
  pc = 0x3100985; continue;
 }
 0x3100985 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31009da; continue; }
  pc = 0x310099c; continue;
 }
 0x310099c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31009d1; continue; }
  pc = 0x31009c0; continue;
 }
 0x31009c0 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31009d1; continue;
 }
 0x31009d1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31009da; continue;
 }
 0x31009da => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x31009f7; continue;
 }
 0x31009e7 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31009f7; continue;
 }
 0x31009f7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31009e7; continue; }
  pc = 0x31009fa; continue;
 }
 0x31009fa => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3100a18; continue;
 }
 0x3100a18 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100a2f; continue;
 }
 0x3100a1f => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100a2f; continue;
 }
 0x3100a2f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100a1f; continue; }
  pc = 0x3100a32; continue;
 }
 0x3100a32 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100a18; continue; }
  pc = 0x3100a3b; continue;
 }
 0x3100a3b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100a90; continue; }
  pc = 0x3100a52; continue;
 }
 0x3100a52 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100a87; continue; }
  pc = 0x3100a76; continue;
 }
 0x3100a76 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100a87; continue;
 }
 0x3100a87 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100a90; continue;
 }
 0x3100a90 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3100a9b; continue;
 }
 0x3100a9b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100ab2; continue;
 }
 0x3100aa2 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100ab2; continue;
 }
 0x3100ab2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100aa2; continue; }
  pc = 0x3100ab5; continue;
 }
 0x3100ab5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100a9b; continue; }
  pc = 0x3100abe; continue;
 }
 0x3100abe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100ad5; continue;
 }
 0x3100ac5 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100ad5; continue;
 }
 0x3100ad5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100ac5; continue; }
  pc = 0x3100ad8; continue;
 }
 0x3100ad8 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100b46; continue; }
  pc = 0x3100b08; continue;
 }
 0x3100b08 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100b3d; continue; }
  pc = 0x3100b2c; continue;
 }
 0x3100b2c => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100b3d; continue;
 }
 0x3100b3d => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100b46; continue;
 }
 0x3100b46 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3100b64; continue;
 }
 0x3100b64 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100b7b; continue;
 }
 0x3100b6b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100b7b; continue;
 }
 0x3100b7b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100b6b; continue; }
  pc = 0x3100b7e; continue;
 }
 0x3100b7e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100b64; continue; }
  pc = 0x3100b87; continue;
 }
 0x3100b87 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x3100b9e; continue;
 }
 0x3100b8e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100b9e; continue;
 }
 0x3100b9e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100b8e; continue; }
  pc = 0x3100ba1; continue;
 }
 0x3100ba1 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100bf6; continue; }
  pc = 0x3100bb8; continue;
 }
 0x3100bb8 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100bed; continue; }
  pc = 0x3100bdc; continue;
 }
 0x3100bdc => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100bed; continue;
 }
 0x3100bed => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100bf6; continue;
 }
 0x3100bf6 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x3100c13; continue;
 }
 0x3100c03 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100c13; continue;
 }
 0x3100c13 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100c03; continue; }
  pc = 0x3100c16; continue;
 }
 0x3100c16 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3100d32; continue; }
  pc = 0x3100c3f; continue;
 }
 0x3100c3f => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x3100c44; continue;
 }
 0x3100c44 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100c5b; continue;
 }
 0x3100c4b => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100c5b; continue;
 }
 0x3100c5b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100c4b; continue; }
  pc = 0x3100c5e; continue;
 }
 0x3100c5e => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100c44; continue; }
  pc = 0x3100c67; continue;
 }
 0x3100c67 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100c7e; continue;
 }
 0x3100c6e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100c7e; continue;
 }
 0x3100c7e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100c6e; continue; }
  pc = 0x3100c81; continue;
 }
 0x3100c81 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100cef; continue; }
  pc = 0x3100cb1; continue;
 }
 0x3100cb1 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100ce6; continue; }
  pc = 0x3100cd5; continue;
 }
 0x3100cd5 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100ce6; continue;
 }
 0x3100ce6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100cef; continue;
 }
 0x3100cef => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x3100d0d; continue;
 }
 0x3100d0d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100d24; continue;
 }
 0x3100d14 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100d24; continue;
 }
 0x3100d24 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100d14; continue; }
  pc = 0x3100d27; continue;
 }
 0x3100d27 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100d0d; continue; }
  pc = 0x3100d30; continue;
 }
 0x3100d30 => {
  pc = 0x3100d5a; continue;
 }
 0x3100d32 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x3100d37; continue;
 }
 0x3100d37 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3100d4e; continue;
 }
 0x3100d3e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3100d4e; continue;
 }
 0x3100d4e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100d3e; continue; }
  pc = 0x3100d51; continue;
 }
 0x3100d51 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100d37; continue; }
  pc = 0x3100d5a; continue;
 }
 0x3100d5a => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03100d5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3100d5c); }
 let mut pc: u32 = 0x3100d5c;
 loop { match pc {
 0x3100d5c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3100d7b; continue; }
  pc = 0x3100d78; continue;
 }
 0x3100d78 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x3100d7b; continue;
 }
 0x3100d7b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100de5; continue; }
  pc = 0x3100da7; continue;
 }
 0x3100da7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100ddc; continue; }
  pc = 0x3100dcb; continue;
 }
 0x3100dcb => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100ddc; continue;
 }
 0x3100ddc => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100de5; continue;
 }
 0x3100de5 => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x3100e20; continue;
 }
 0x3100e20 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x3100e47; continue; }
  pc = 0x3100e28; continue;
 }
 0x3100e28 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3100e36; continue;
 }
 0x3100e2d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3100e36; continue;
 }
 0x3100e36 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e2d; continue; }
  pc = 0x3100e39; continue;
 }
 0x3100e39 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e20; continue; }
  pc = 0x3100e42; continue;
 }
 0x3100e42 => {
  pc = 0x310110c; continue;
 }
 0x3100e47 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x3100f13; continue; }
  pc = 0x3100e58; continue;
 }
 0x3100e58 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3100e66; continue;
 }
 0x3100e5d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3100e66; continue;
 }
 0x3100e66 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e5d; continue; }
  pc = 0x3100e69; continue;
 }
 0x3100e69 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100ec0; continue; }
  pc = 0x3100e82; continue;
 }
 0x3100e82 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100eb7; continue; }
  pc = 0x3100ea6; continue;
 }
 0x3100ea6 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100eb7; continue;
 }
 0x3100eb7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100ec0; continue;
 }
 0x3100ec0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3100ee0; continue;
 }
 0x3100ed7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3100ee0; continue;
 }
 0x3100ee0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100ed7; continue; }
  pc = 0x3100ee3; continue;
 }
 0x3100ee3 => {
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
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e20; continue; }
  pc = 0x3100f0e; continue;
 }
 0x3100f0e => {
  pc = 0x310110c; continue;
 }
 0x3100f13 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x310106f; continue; }
  pc = 0x3100f21; continue;
 }
 0x3100f21 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x3100feb; continue; }
  pc = 0x3100f2a; continue;
 }
 0x3100f2a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3100f38; continue;
 }
 0x3100f2f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3100f38; continue;
 }
 0x3100f38 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100f2f; continue; }
  pc = 0x3100f3b; continue;
 }
 0x3100f3b => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3100fb2; continue; }
  pc = 0x3100f74; continue;
 }
 0x3100f74 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3100fa9; continue; }
  pc = 0x3100f98; continue;
 }
 0x3100f98 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3100fa9; continue;
 }
 0x3100fa9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3100fb2; continue;
 }
 0x3100fb2 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e20; continue; }
  pc = 0x3100fe6; continue;
 }
 0x3100fe6 => {
  pc = 0x310110c; continue;
 }
 0x3100feb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3100ff9; continue;
 }
 0x3100ff0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3100ff9; continue;
 }
 0x3100ff9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100ff0; continue; }
  pc = 0x3100ffc; continue;
 }
 0x3100ffc => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101053; continue; }
  pc = 0x3101015; continue;
 }
 0x3101015 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310104a; continue; }
  pc = 0x3101039; continue;
 }
 0x3101039 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310104a; continue;
 }
 0x310104a => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101053; continue;
 }
 0x3101053 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e20; continue; }
  pc = 0x310106a; continue;
 }
 0x310106a => {
  pc = 0x310110c; continue;
 }
 0x310106f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x310107d; continue;
 }
 0x3101074 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x310107d; continue;
 }
 0x310107d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101074; continue; }
  pc = 0x3101080; continue;
 }
 0x3101080 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31010dd; continue; }
  pc = 0x310109f; continue;
 }
 0x310109f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31010d4; continue; }
  pc = 0x31010c3; continue;
 }
 0x31010c3 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31010d4; continue;
 }
 0x31010d4 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31010dd; continue;
 }
 0x31010dd => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3100e20; continue; }
  pc = 0x310110c; continue;
 }
 0x310110c => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310110f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310110f); }
 let mut pc: u32 = 0x310110f;
 loop { match pc {
 0x310110f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x310112e; continue; }
  pc = 0x310112b; continue;
 }
 0x310112b => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x310112e; continue;
 }
 0x310112e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101198; continue; }
  pc = 0x310115a; continue;
 }
 0x310115a => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310118f; continue; }
  pc = 0x310117e; continue;
 }
 0x310117e => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310118f; continue;
 }
 0x310118f => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101198; continue;
 }
 0x3101198 => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x31011d3; continue;
 }
 0x31011d3 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x31011f4; continue; }
  pc = 0x31011db; continue;
 }
 0x31011db => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31011e3; continue;
 }
 0x31011e0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31011e3; continue;
 }
 0x31011e3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011e0; continue; }
  pc = 0x31011e6; continue;
 }
 0x31011e6 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011d3; continue; }
  pc = 0x31011ef; continue;
 }
 0x31011ef => {
  pc = 0x310148b; continue;
 }
 0x31011f4 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x31012b0; continue; }
  pc = 0x3101205; continue;
 }
 0x3101205 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x310120d; continue;
 }
 0x310120a => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x310120d; continue;
 }
 0x310120d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310120a; continue; }
  pc = 0x3101210; continue;
 }
 0x3101210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101267; continue; }
  pc = 0x3101229; continue;
 }
 0x3101229 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310125e; continue; }
  pc = 0x310124d; continue;
 }
 0x310124d => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310125e; continue;
 }
 0x310125e => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101267; continue;
 }
 0x3101267 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3101281; continue;
 }
 0x310127e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3101281; continue;
 }
 0x3101281 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310127e; continue; }
  pc = 0x3101284; continue;
 }
 0x3101284 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101295; continue;
 }
 0x3101292 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3101295; continue;
 }
 0x3101295 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101292; continue; }
  pc = 0x3101298; continue;
 }
 0x3101298 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011d3; continue; }
  pc = 0x31012ab; continue;
 }
 0x31012ab => {
  pc = 0x310148b; continue;
 }
 0x31012b0 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x31013f8; continue; }
  pc = 0x31012be; continue;
 }
 0x31012be => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x310137a; continue; }
  pc = 0x31012c7; continue;
 }
 0x31012c7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31012cf; continue;
 }
 0x31012cc => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31012cf; continue;
 }
 0x31012cf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31012cc; continue; }
  pc = 0x31012d2; continue;
 }
 0x31012d2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x31012eb; continue;
 }
 0x31012e8 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31012eb; continue;
 }
 0x31012eb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31012e8; continue; }
  pc = 0x31012ee; continue;
 }
 0x31012ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101345; continue; }
  pc = 0x3101307; continue;
 }
 0x3101307 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310133c; continue; }
  pc = 0x310132b; continue;
 }
 0x310132b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310133c; continue;
 }
 0x310133c => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101345; continue;
 }
 0x3101345 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x310135f; continue;
 }
 0x310135c => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x310135f; continue;
 }
 0x310135f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310135c; continue; }
  pc = 0x3101362; continue;
 }
 0x3101362 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011d3; continue; }
  pc = 0x3101375; continue;
 }
 0x3101375 => {
  pc = 0x310148b; continue;
 }
 0x310137a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101382; continue;
 }
 0x310137f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3101382; continue;
 }
 0x3101382 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310137f; continue; }
  pc = 0x3101385; continue;
 }
 0x3101385 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31013dc; continue; }
  pc = 0x310139e; continue;
 }
 0x310139e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31013d3; continue; }
  pc = 0x31013c2; continue;
 }
 0x31013c2 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31013d3; continue;
 }
 0x31013d3 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31013dc; continue;
 }
 0x31013dc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011d3; continue; }
  pc = 0x31013f3; continue;
 }
 0x31013f3 => {
  pc = 0x310148b; continue;
 }
 0x31013f8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101400; continue;
 }
 0x31013fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3101400; continue;
 }
 0x3101400 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31013fd; continue; }
  pc = 0x3101403; continue;
 }
 0x3101403 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101460; continue; }
  pc = 0x3101422; continue;
 }
 0x3101422 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101457; continue; }
  pc = 0x3101446; continue;
 }
 0x3101446 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101457; continue;
 }
 0x3101457 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101460; continue;
 }
 0x3101460 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x310147b; continue;
 }
 0x3101478 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x310147b; continue;
 }
 0x310147b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101478; continue; }
  pc = 0x310147e; continue;
 }
 0x310147e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31011d3; continue; }
  pc = 0x310148b; continue;
 }
 0x310148b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310148e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310148e); }
 let mut pc: u32 = 0x310148e;
 loop { match pc {
 0x310148e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31014de; continue; }
  pc = 0x31014a0; continue;
 }
 0x31014a0 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31014d5; continue; }
  pc = 0x31014c4; continue;
 }
 0x31014c4 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31014d5; continue;
 }
 0x31014d5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31014de; continue;
 }
 0x31014de => {
  { let v: u32 = cpu.edi; cpu.w32(0x32f1e80u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3101500; continue; }
  pc = 0x31014f5; continue;
 }
 0x31014f5 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x3101500; continue;
 }
 0x3101500 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101523; continue;
 }
 0x3101507 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101523; continue;
 }
 0x3101523 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101507; continue; }
  pc = 0x3101526; continue;
 }
 0x3101526 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101500; continue; }
  pc = 0x310152f; continue;
 }
 0x310152f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101546; continue;
 }
 0x3101536 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101546; continue;
 }
 0x3101546 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101536; continue; }
  pc = 0x3101549; continue;
 }
 0x3101549 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x3101566; continue;
 }
 0x3101556 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101566; continue;
 }
 0x3101566 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101556; continue; }
  pc = 0x3101569; continue;
 }
 0x3101569 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31015be; continue; }
  pc = 0x3101580; continue;
 }
 0x3101580 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31015b5; continue; }
  pc = 0x31015a4; continue;
 }
 0x31015a4 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31015b5; continue;
 }
 0x31015b5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31015be; continue;
 }
 0x31015be => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x31015db; continue;
 }
 0x31015cb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31015db; continue;
 }
 0x31015db => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31015cb; continue; }
  pc = 0x31015de; continue;
 }
 0x31015de => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x31015e3; continue;
 }
 0x31015e3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101606; continue;
 }
 0x31015ea => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101606; continue;
 }
 0x3101606 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31015ea; continue; }
  pc = 0x3101609; continue;
 }
 0x3101609 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31015e3; continue; }
  pc = 0x3101612; continue;
 }
 0x3101612 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x3101629; continue;
 }
 0x3101619 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101629; continue;
 }
 0x3101629 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101619; continue; }
  pc = 0x310162c; continue;
 }
 0x310162c => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101681; continue; }
  pc = 0x3101643; continue;
 }
 0x3101643 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101678; continue; }
  pc = 0x3101667; continue;
 }
 0x3101667 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101678; continue;
 }
 0x3101678 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101681; continue;
 }
 0x3101681 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x310169e; continue;
 }
 0x310168e => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310169e; continue;
 }
 0x310169e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310168e; continue; }
  pc = 0x31016a1; continue;
 }
 0x31016a1 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31016be; continue;
 }
 0x31016ae => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31016be; continue;
 }
 0x31016be => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31016ae; continue; }
  pc = 0x31016c1; continue;
 }
 0x31016c1 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x31016c6; continue;
 }
 0x31016c6 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31016e9; continue;
 }
 0x31016cd => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31016e9; continue;
 }
 0x31016e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31016cd; continue; }
  pc = 0x31016ec; continue;
 }
 0x31016ec => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31016c6; continue; }
  pc = 0x31016f5; continue;
 }
 0x31016f5 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x310170c; continue;
 }
 0x31016fc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310170c; continue;
 }
 0x310170c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31016fc; continue; }
  pc = 0x310170f; continue;
 }
 0x310170f => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x310172c; continue;
 }
 0x310171c => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310172c; continue;
 }
 0x310172c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310171c; continue; }
  pc = 0x310172f; continue;
 }
 0x310172f => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101784; continue; }
  pc = 0x3101746; continue;
 }
 0x3101746 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310177b; continue; }
  pc = 0x310176a; continue;
 }
 0x310176a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310177b; continue;
 }
 0x310177b => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101784; continue;
 }
 0x3101784 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x31017a1; continue;
 }
 0x3101791 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31017a1; continue;
 }
 0x31017a1 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101791; continue; }
  pc = 0x31017a4; continue;
 }
 0x31017a4 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x31017a9; continue;
 }
 0x31017a9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31017cc; continue;
 }
 0x31017b0 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31017cc; continue;
 }
 0x31017cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31017b0; continue; }
  pc = 0x31017cf; continue;
 }
 0x31017cf => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31017a9; continue; }
  pc = 0x31017d8; continue;
 }
 0x31017d8 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x31017ef; continue;
 }
 0x31017df => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31017ef; continue;
 }
 0x31017ef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31017df; continue; }
  pc = 0x31017f2; continue;
 }
 0x31017f2 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101847; continue; }
  pc = 0x3101809; continue;
 }
 0x3101809 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310183e; continue; }
  pc = 0x310182d; continue;
 }
 0x310182d => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310183e; continue;
 }
 0x310183e => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101847; continue;
 }
 0x3101847 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x3101864; continue;
 }
 0x3101854 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101864; continue;
 }
 0x3101864 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101854; continue; }
  pc = 0x3101867; continue;
 }
 0x3101867 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101884; continue;
 }
 0x3101874 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101884; continue;
 }
 0x3101884 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101874; continue; }
  pc = 0x3101887; continue;
 }
 0x3101887 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x310188c; continue;
 }
 0x310188c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x31018af; continue;
 }
 0x3101893 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31018af; continue;
 }
 0x31018af => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101893; continue; }
  pc = 0x31018b2; continue;
 }
 0x31018b2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310188c; continue; }
  pc = 0x31018bb; continue;
 }
 0x31018bb => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101910; continue; }
  pc = 0x31018d2; continue;
 }
 0x31018d2 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101907; continue; }
  pc = 0x31018f6; continue;
 }
 0x31018f6 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101907; continue;
 }
 0x3101907 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101910; continue;
 }
 0x3101910 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x310191b; continue;
 }
 0x310191b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x310193e; continue;
 }
 0x3101922 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x310193e; continue;
 }
 0x310193e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101922; continue; }
  pc = 0x3101941; continue;
 }
 0x3101941 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310191b; continue; }
  pc = 0x310194a; continue;
 }
 0x310194a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101961; continue;
 }
 0x3101951 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101961; continue;
 }
 0x3101961 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101951; continue; }
  pc = 0x3101964; continue;
 }
 0x3101964 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  pc = 0x3101981; continue;
 }
 0x3101971 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101981; continue;
 }
 0x3101981 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101971; continue; }
  pc = 0x3101984; continue;
 }
 0x3101984 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31019d9; continue; }
  pc = 0x310199b; continue;
 }
 0x310199b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31019d0; continue; }
  pc = 0x31019bf; continue;
 }
 0x31019bf => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31019d0; continue;
 }
 0x31019d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31019d9; continue;
 }
 0x31019d9 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  pc = 0x31019f6; continue;
 }
 0x31019e6 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x31019f6; continue;
 }
 0x31019f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31019e6; continue; }
  pc = 0x31019f9; continue;
 }
 0x31019f9 => {
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x31019fe; continue;
 }
 0x31019fe => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101a21; continue;
 }
 0x3101a05 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101a21; continue;
 }
 0x3101a21 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101a05; continue; }
  pc = 0x3101a24; continue;
 }
 0x3101a24 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31019fe; continue; }
  pc = 0x3101a2d; continue;
 }
 0x3101a2d => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x3101a44; continue;
 }
 0x3101a34 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101a44; continue;
 }
 0x3101a44 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101a34; continue; }
  pc = 0x3101a47; continue;
 }
 0x3101a47 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101a9c; continue; }
  pc = 0x3101a5e; continue;
 }
 0x3101a5e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101a93; continue; }
  pc = 0x3101a82; continue;
 }
 0x3101a82 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101a93; continue;
 }
 0x3101a93 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101a9c; continue;
 }
 0x3101a9c => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x3101ab9; continue;
 }
 0x3101aa9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101ab9; continue;
 }
 0x3101ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101aa9; continue; }
  pc = 0x3101abc; continue;
 }
 0x3101abc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101ad9; continue;
 }
 0x3101ac9 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101ad9; continue;
 }
 0x3101ad9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101ac9; continue; }
  pc = 0x3101adc; continue;
 }
 0x3101adc => {
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3101c05; continue; }
  pc = 0x3101aec; continue;
 }
 0x3101aec => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x3101af1; continue;
 }
 0x3101af1 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101b14; continue;
 }
 0x3101af8 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101b14; continue;
 }
 0x3101b14 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101af8; continue; }
  pc = 0x3101b17; continue;
 }
 0x3101b17 => {
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101af1; continue; }
  pc = 0x3101b20; continue;
 }
 0x3101b20 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101b37; continue;
 }
 0x3101b27 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101b37; continue;
 }
 0x3101b37 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101b27; continue; }
  pc = 0x3101b3a; continue;
 }
 0x3101b3a => {
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  pc = 0x3101b57; continue;
 }
 0x3101b47 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101b57; continue;
 }
 0x3101b57 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101b47; continue; }
  pc = 0x3101b5a; continue;
 }
 0x3101b5a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101baf; continue; }
  pc = 0x3101b71; continue;
 }
 0x3101b71 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101ba6; continue; }
  pc = 0x3101b95; continue;
 }
 0x3101b95 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101ba6; continue;
 }
 0x3101ba6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101baf; continue;
 }
 0x3101baf => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  pc = 0x3101bcc; continue;
 }
 0x3101bbc => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101bcc; continue;
 }
 0x3101bcc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101bbc; continue; }
  pc = 0x3101bcf; continue;
 }
 0x3101bcf => {
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x3101bd4; continue;
 }
 0x3101bd4 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101bf7; continue;
 }
 0x3101bdb => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101bf7; continue;
 }
 0x3101bf7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101bdb; continue; }
  pc = 0x3101bfa; continue;
 }
 0x3101bfa => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101bd4; continue; }
  pc = 0x3101c03; continue;
 }
 0x3101c03 => {
  pc = 0x3101c39; continue;
 }
 0x3101c05 => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x3101c0a; continue;
 }
 0x3101c0a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  pc = 0x3101c2d; continue;
 }
 0x3101c11 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3101c2d; continue;
 }
 0x3101c2d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101c11; continue; }
  pc = 0x3101c30; continue;
 }
 0x3101c30 => {
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101c0a; continue; }
  pc = 0x3101c39; continue;
 }
 0x3101c39 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03101c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3101c3b); }
 let mut pc: u32 = 0x3101c3b;
 loop { match pc {
 0x3101c3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3101c5a; continue; }
  pc = 0x3101c57; continue;
 }
 0x3101c57 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x3101c5a; continue;
 }
 0x3101c5a => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101cc4; continue; }
  pc = 0x3101c86; continue;
 }
 0x3101c86 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101cbb; continue; }
  pc = 0x3101caa; continue;
 }
 0x3101caa => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101cbb; continue;
 }
 0x3101cbb => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101cc4; continue;
 }
 0x3101cc4 => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x3101cff; continue;
 }
 0x3101cff => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x3101d2c; continue; }
  pc = 0x3101d07; continue;
 }
 0x3101d07 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101d1b; continue;
 }
 0x3101d0c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x3101d1b; continue;
 }
 0x3101d1b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101d0c; continue; }
  pc = 0x3101d1e; continue;
 }
 0x3101d1e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101cff; continue; }
  pc = 0x3101d27; continue;
 }
 0x3101d27 => {
  pc = 0x3101fff; continue;
 }
 0x3101d2c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x3101dfa; continue; }
  pc = 0x3101d3d; continue;
 }
 0x3101d3d => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3101d4b; continue;
 }
 0x3101d42 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101d4b; continue;
 }
 0x3101d4b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101d42; continue; }
  pc = 0x3101d4e; continue;
 }
 0x3101d4e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101da5; continue; }
  pc = 0x3101d67; continue;
 }
 0x3101d67 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101d9c; continue; }
  pc = 0x3101d8b; continue;
 }
 0x3101d8b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101d9c; continue;
 }
 0x3101d9c => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101da5; continue;
 }
 0x3101da5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3101dc5; continue;
 }
 0x3101dbc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101dc5; continue;
 }
 0x3101dc5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101dbc; continue; }
  pc = 0x3101dc8; continue;
 }
 0x3101dc8 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101ddf; continue;
 }
 0x3101dd6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101ddf; continue;
 }
 0x3101ddf => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101dd6; continue; }
  pc = 0x3101de2; continue;
 }
 0x3101de2 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101cff; continue; }
  pc = 0x3101df5; continue;
 }
 0x3101df5 => {
  pc = 0x3101fff; continue;
 }
 0x3101dfa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x3101f60; continue; }
  pc = 0x3101e08; continue;
 }
 0x3101e08 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x3101ed6; continue; }
  pc = 0x3101e11; continue;
 }
 0x3101e11 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101e1f; continue;
 }
 0x3101e16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101e1f; continue;
 }
 0x3101e1f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101e16; continue; }
  pc = 0x3101e22; continue;
 }
 0x3101e22 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3101e41; continue;
 }
 0x3101e38 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101e41; continue;
 }
 0x3101e41 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101e38; continue; }
  pc = 0x3101e44; continue;
 }
 0x3101e44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101e9b; continue; }
  pc = 0x3101e5d; continue;
 }
 0x3101e5d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101e92; continue; }
  pc = 0x3101e81; continue;
 }
 0x3101e81 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101e92; continue;
 }
 0x3101e92 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101e9b; continue;
 }
 0x3101e9b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3101ebb; continue;
 }
 0x3101eb2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101ebb; continue;
 }
 0x3101ebb => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101eb2; continue; }
  pc = 0x3101ebe; continue;
 }
 0x3101ebe => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101cff; continue; }
  pc = 0x3101ed1; continue;
 }
 0x3101ed1 => {
  pc = 0x3101fff; continue;
 }
 0x3101ed6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101eea; continue;
 }
 0x3101edb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x3101eea; continue;
 }
 0x3101eea => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101edb; continue; }
  pc = 0x3101eed; continue;
 }
 0x3101eed => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101f44; continue; }
  pc = 0x3101f06; continue;
 }
 0x3101f06 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101f3b; continue; }
  pc = 0x3101f2a; continue;
 }
 0x3101f2a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101f3b; continue;
 }
 0x3101f3b => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101f44; continue;
 }
 0x3101f44 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101cff; continue; }
  pc = 0x3101f5b; continue;
 }
 0x3101f5b => {
  pc = 0x3101fff; continue;
 }
 0x3101f60 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101f6e; continue;
 }
 0x3101f65 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101f6e; continue;
 }
 0x3101f6e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101f65; continue; }
  pc = 0x3101f71; continue;
 }
 0x3101f71 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3101fce; continue; }
  pc = 0x3101f90; continue;
 }
 0x3101f90 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3101fc5; continue; }
  pc = 0x3101fb4; continue;
 }
 0x3101fb4 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3101fc5; continue;
 }
 0x3101fc5 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3101fce; continue;
 }
 0x3101fce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3101fef; continue;
 }
 0x3101fe6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  pc = 0x3101fef; continue;
 }
 0x3101fef => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101fe6; continue; }
  pc = 0x3101ff2; continue;
 }
 0x3101ff2 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3101cff; continue; }
  pc = 0x3101fff; continue;
 }
 0x3101fff => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03102002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3102002); }
 let mut pc: u32 = 0x3102002;
 loop { match pc {
 0x3102002 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3102021; continue; }
  pc = 0x310201e; continue;
 }
 0x310201e => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x3102021; continue;
 }
 0x3102021 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310208b; continue; }
  pc = 0x310204d; continue;
 }
 0x310204d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3102082; continue; }
  pc = 0x3102071; continue;
 }
 0x3102071 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3102082; continue;
 }
 0x3102082 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310208b; continue;
 }
 0x310208b => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x31020c6; continue;
 }
 0x31020c6 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x31020e7; continue; }
  pc = 0x31020ce; continue;
 }
 0x31020ce => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31020d6; continue;
 }
 0x31020d3 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31020d6; continue;
 }
 0x31020d6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020d3; continue; }
  pc = 0x31020d9; continue;
 }
 0x31020d9 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020c6; continue; }
  pc = 0x31020e2; continue;
 }
 0x31020e2 => {
  pc = 0x310237e; continue;
 }
 0x31020e7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x31021a3; continue; }
  pc = 0x31020f8; continue;
 }
 0x31020f8 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3102100; continue;
 }
 0x31020fd => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3102100; continue;
 }
 0x3102100 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020fd; continue; }
  pc = 0x3102103; continue;
 }
 0x3102103 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310215a; continue; }
  pc = 0x310211c; continue;
 }
 0x310211c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3102151; continue; }
  pc = 0x3102140; continue;
 }
 0x3102140 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3102151; continue;
 }
 0x3102151 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310215a; continue;
 }
 0x310215a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3102174; continue;
 }
 0x3102171 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3102174; continue;
 }
 0x3102174 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102171; continue; }
  pc = 0x3102177; continue;
 }
 0x3102177 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3102188; continue;
 }
 0x3102185 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3102188; continue;
 }
 0x3102188 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102185; continue; }
  pc = 0x310218b; continue;
 }
 0x310218b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020c6; continue; }
  pc = 0x310219e; continue;
 }
 0x310219e => {
  pc = 0x310237e; continue;
 }
 0x31021a3 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x31022eb; continue; }
  pc = 0x31021b1; continue;
 }
 0x31021b1 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x310226d; continue; }
  pc = 0x31021ba; continue;
 }
 0x31021ba => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31021c2; continue;
 }
 0x31021bf => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31021c2; continue;
 }
 0x31021c2 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31021bf; continue; }
  pc = 0x31021c5; continue;
 }
 0x31021c5 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x31021de; continue;
 }
 0x31021db => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31021de; continue;
 }
 0x31021de => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31021db; continue; }
  pc = 0x31021e1; continue;
 }
 0x31021e1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3102238; continue; }
  pc = 0x31021fa; continue;
 }
 0x31021fa => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310222f; continue; }
  pc = 0x310221e; continue;
 }
 0x310221e => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310222f; continue;
 }
 0x310222f => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3102238; continue;
 }
 0x3102238 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3102252; continue;
 }
 0x310224f => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3102252; continue;
 }
 0x3102252 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310224f; continue; }
  pc = 0x3102255; continue;
 }
 0x3102255 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020c6; continue; }
  pc = 0x3102268; continue;
 }
 0x3102268 => {
  pc = 0x310237e; continue;
 }
 0x310226d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3102275; continue;
 }
 0x3102272 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3102275; continue;
 }
 0x3102275 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102272; continue; }
  pc = 0x3102278; continue;
 }
 0x3102278 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31022cf; continue; }
  pc = 0x3102291; continue;
 }
 0x3102291 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31022c6; continue; }
  pc = 0x31022b5; continue;
 }
 0x31022b5 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31022c6; continue;
 }
 0x31022c6 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31022cf; continue;
 }
 0x31022cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020c6; continue; }
  pc = 0x31022e6; continue;
 }
 0x31022e6 => {
  pc = 0x310237e; continue;
 }
 0x31022eb => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31022f3; continue;
 }
 0x31022f0 => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x31022f3; continue;
 }
 0x31022f3 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31022f0; continue; }
  pc = 0x31022f6; continue;
 }
 0x31022f6 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3102353; continue; }
  pc = 0x3102315; continue;
 }
 0x3102315 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310234a; continue; }
  pc = 0x3102339; continue;
 }
 0x3102339 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310234a; continue;
 }
 0x310234a => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3102353; continue;
 }
 0x3102353 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x310236e; continue;
 }
 0x310236b => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x310236e; continue;
 }
 0x310236e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310236b; continue; }
  pc = 0x3102371; continue;
 }
 0x3102371 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31020c6; continue; }
  pc = 0x310237e; continue;
 }
 0x310237e => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03102381(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3102381); }
 let mut pc: u32 = 0x3102381;
 loop { match pc {
 0x3102381 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffffcu32); cpu.esp = v; }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31023d1; continue; }
  pc = 0x3102393; continue;
 }
 0x3102393 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31023c8; continue; }
  pc = 0x31023b7; continue;
 }
 0x31023b7 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31023c8; continue;
 }
 0x31023c8 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31023d1; continue;
 }
 0x31023d1 => {
  { let v: u32 = cpu.edi; cpu.w32(0x32f1e80u32, v); }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x31023f3; continue; }
  pc = 0x31023e8; continue;
 }
 0x31023e8 => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  { let v: u32 = 0x5u32; cpu.edx = v; }
  pc = 0x31023f3; continue;
 }
 0x31023f3 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31023fb, "Movq");
  cpu.unimplemented(0x3102402, "Movq");
  pc = 0x3102405; continue;
 }
 0x3102405 => {
  cpu.unimplemented(0x3102405, "Movq");
  cpu.unimplemented(0x3102409, "Movq");
  cpu.unimplemented(0x310240c, "Movq");
  cpu.unimplemented(0x310240f, "Movq");
  cpu.unimplemented(0x3102412, "Psrlq");
  cpu.unimplemented(0x3102416, "Psllq");
  cpu.unimplemented(0x310241a, "Pand");
  cpu.unimplemented(0x3102421, "Por");
  cpu.unimplemented(0x3102424, "Psrlq");
  cpu.unimplemented(0x3102428, "Pand");
  cpu.unimplemented(0x310242f, "Psrlq");
  cpu.unimplemented(0x3102433, "Paddw");
  cpu.unimplemented(0x3102436, "Movq");
  cpu.unimplemented(0x3102439, "Punpcklwd");
  cpu.unimplemented(0x310243c, "Punpckhwd");
  cpu.unimplemented(0x310243f, "Movq");
  cpu.unimplemented(0x3102442, "Pand");
  cpu.unimplemented(0x3102445, "Movq");
  cpu.unimplemented(0x310244c, "Movq");
  cpu.unimplemented(0x3102450, "Pand");
  cpu.unimplemented(0x3102453, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102460, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102405; continue; }
  pc = 0x3102466; continue;
 }
 0x3102466 => {
  cpu.unimplemented(0x3102466, "Punpcklwd");
  cpu.unimplemented(0x3102469, "Punpckhwd");
  cpu.unimplemented(0x310246c, "Movq");
  cpu.unimplemented(0x310246f, "Pand");
  cpu.unimplemented(0x3102472, "Movq");
  cpu.unimplemented(0x3102479, "Movq");
  cpu.unimplemented(0x310247d, "Pand");
  cpu.unimplemented(0x3102480, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31023f3; continue; }
  pc = 0x310249a; continue;
 }
 0x310249a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31024a2, "Movq");
  cpu.unimplemented(0x31024a9, "Movq");
  pc = 0x31024ac; continue;
 }
 0x31024ac => {
  cpu.unimplemented(0x31024ac, "Movq");
  cpu.unimplemented(0x31024b0, "Movq");
  cpu.unimplemented(0x31024b3, "Movq");
  cpu.unimplemented(0x31024b6, "Movq");
  cpu.unimplemented(0x31024b9, "Psrlq");
  cpu.unimplemented(0x31024bd, "Psllq");
  cpu.unimplemented(0x31024c1, "Pand");
  cpu.unimplemented(0x31024c8, "Por");
  cpu.unimplemented(0x31024cb, "Psrlq");
  cpu.unimplemented(0x31024cf, "Pand");
  cpu.unimplemented(0x31024d6, "Psrlq");
  cpu.unimplemented(0x31024da, "Paddw");
  cpu.unimplemented(0x31024dd, "Movq");
  cpu.unimplemented(0x31024e0, "Punpcklwd");
  cpu.unimplemented(0x31024e3, "Punpckhwd");
  cpu.unimplemented(0x31024e6, "Movq");
  cpu.unimplemented(0x31024e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31024f3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31024ac; continue; }
  pc = 0x31024f9; continue;
 }
 0x31024f9 => {
  cpu.unimplemented(0x31024f9, "Punpcklwd");
  cpu.unimplemented(0x31024fc, "Punpckhwd");
  cpu.unimplemented(0x31024ff, "Movq");
  cpu.unimplemented(0x3102502, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x310251a, "Movq");
  cpu.unimplemented(0x3102521, "Movq");
  pc = 0x3102524; continue;
 }
 0x3102524 => {
  cpu.unimplemented(0x3102524, "Movq");
  cpu.unimplemented(0x3102528, "Movq");
  cpu.unimplemented(0x310252b, "Movq");
  cpu.unimplemented(0x310252e, "Movq");
  cpu.unimplemented(0x3102531, "Psrlq");
  cpu.unimplemented(0x3102535, "Psllq");
  cpu.unimplemented(0x3102539, "Pand");
  cpu.unimplemented(0x3102540, "Por");
  cpu.unimplemented(0x3102543, "Psrlq");
  cpu.unimplemented(0x3102547, "Pand");
  cpu.unimplemented(0x310254e, "Psrlq");
  cpu.unimplemented(0x3102552, "Paddw");
  cpu.unimplemented(0x3102555, "Movq");
  cpu.unimplemented(0x3102558, "Punpcklwd");
  cpu.unimplemented(0x310255b, "Punpckhwd");
  cpu.unimplemented(0x310255e, "Movq");
  cpu.unimplemented(0x3102561, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x310256b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102524; continue; }
  pc = 0x3102571; continue;
 }
 0x3102571 => {
  cpu.unimplemented(0x3102571, "Punpcklwd");
  cpu.unimplemented(0x3102574, "Punpckhwd");
  cpu.unimplemented(0x3102577, "Movq");
  cpu.unimplemented(0x310257a, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31025d9; continue; }
  pc = 0x310259b; continue;
 }
 0x310259b => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31025d0; continue; }
  pc = 0x31025bf; continue;
 }
 0x31025bf => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31025d0; continue;
 }
 0x31025d0 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31025d9; continue;
 }
 0x31025d9 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31025e7, "Movq");
  cpu.unimplemented(0x31025ee, "Movq");
  pc = 0x31025f1; continue;
 }
 0x31025f1 => {
  cpu.unimplemented(0x31025f1, "Movq");
  cpu.unimplemented(0x31025f5, "Movq");
  cpu.unimplemented(0x31025f8, "Movq");
  cpu.unimplemented(0x31025fb, "Movq");
  cpu.unimplemented(0x31025fe, "Psrlq");
  cpu.unimplemented(0x3102602, "Psllq");
  cpu.unimplemented(0x3102606, "Pand");
  cpu.unimplemented(0x310260d, "Por");
  cpu.unimplemented(0x3102610, "Psrlq");
  cpu.unimplemented(0x3102614, "Pand");
  cpu.unimplemented(0x310261b, "Psrlq");
  cpu.unimplemented(0x310261f, "Paddw");
  cpu.unimplemented(0x3102622, "Movq");
  cpu.unimplemented(0x3102625, "Punpcklwd");
  cpu.unimplemented(0x3102628, "Punpckhwd");
  cpu.unimplemented(0x310262b, "Movq");
  cpu.unimplemented(0x310262e, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102638, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31025f1; continue; }
  pc = 0x310263e; continue;
 }
 0x310263e => {
  cpu.unimplemented(0x310263e, "Punpcklwd");
  cpu.unimplemented(0x3102641, "Punpckhwd");
  cpu.unimplemented(0x3102644, "Movq");
  cpu.unimplemented(0x3102647, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3102656; continue;
 }
 0x3102656 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x310265e, "Movq");
  cpu.unimplemented(0x3102665, "Movq");
  pc = 0x3102668; continue;
 }
 0x3102668 => {
  cpu.unimplemented(0x3102668, "Movq");
  cpu.unimplemented(0x310266c, "Movq");
  cpu.unimplemented(0x310266f, "Movq");
  cpu.unimplemented(0x3102672, "Movq");
  cpu.unimplemented(0x3102675, "Psrlq");
  cpu.unimplemented(0x3102679, "Psllq");
  cpu.unimplemented(0x310267d, "Pand");
  cpu.unimplemented(0x3102684, "Por");
  cpu.unimplemented(0x3102687, "Psrlq");
  cpu.unimplemented(0x310268b, "Pand");
  cpu.unimplemented(0x3102692, "Psrlq");
  cpu.unimplemented(0x3102696, "Paddw");
  cpu.unimplemented(0x3102699, "Movq");
  cpu.unimplemented(0x310269c, "Punpcklwd");
  cpu.unimplemented(0x310269f, "Punpckhwd");
  cpu.unimplemented(0x31026a2, "Movq");
  cpu.unimplemented(0x31026a5, "Pand");
  cpu.unimplemented(0x31026a8, "Movq");
  cpu.unimplemented(0x31026af, "Movq");
  cpu.unimplemented(0x31026b3, "Pand");
  cpu.unimplemented(0x31026b6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31026c3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102668; continue; }
  pc = 0x31026c9; continue;
 }
 0x31026c9 => {
  cpu.unimplemented(0x31026c9, "Punpcklwd");
  cpu.unimplemented(0x31026cc, "Punpckhwd");
  cpu.unimplemented(0x31026cf, "Movq");
  cpu.unimplemented(0x31026d2, "Pand");
  cpu.unimplemented(0x31026d5, "Movq");
  cpu.unimplemented(0x31026dc, "Movq");
  cpu.unimplemented(0x31026e0, "Pand");
  cpu.unimplemented(0x31026e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102656; continue; }
  pc = 0x31026fd; continue;
 }
 0x31026fd => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102705, "Movq");
  cpu.unimplemented(0x310270c, "Movq");
  pc = 0x310270f; continue;
 }
 0x310270f => {
  cpu.unimplemented(0x310270f, "Movq");
  cpu.unimplemented(0x3102713, "Movq");
  cpu.unimplemented(0x3102716, "Movq");
  cpu.unimplemented(0x3102719, "Movq");
  cpu.unimplemented(0x310271c, "Psrlq");
  cpu.unimplemented(0x3102720, "Psllq");
  cpu.unimplemented(0x3102724, "Pand");
  cpu.unimplemented(0x310272b, "Por");
  cpu.unimplemented(0x310272e, "Psrlq");
  cpu.unimplemented(0x3102732, "Pand");
  cpu.unimplemented(0x3102739, "Psrlq");
  cpu.unimplemented(0x310273d, "Paddw");
  cpu.unimplemented(0x3102740, "Movq");
  cpu.unimplemented(0x3102743, "Punpcklwd");
  cpu.unimplemented(0x3102746, "Punpckhwd");
  cpu.unimplemented(0x3102749, "Movq");
  cpu.unimplemented(0x310274c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102756, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x310270f; continue; }
  pc = 0x310275c; continue;
 }
 0x310275c => {
  cpu.unimplemented(0x310275c, "Punpcklwd");
  cpu.unimplemented(0x310275f, "Punpckhwd");
  cpu.unimplemented(0x3102762, "Movq");
  cpu.unimplemented(0x3102765, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31027c4; continue; }
  pc = 0x3102786; continue;
 }
 0x3102786 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31027bb; continue; }
  pc = 0x31027aa; continue;
 }
 0x31027aa => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31027bb; continue;
 }
 0x31027bb => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31027c4; continue;
 }
 0x31027c4 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31027d2, "Movq");
  cpu.unimplemented(0x31027d9, "Movq");
  pc = 0x31027dc; continue;
 }
 0x31027dc => {
  cpu.unimplemented(0x31027dc, "Movq");
  cpu.unimplemented(0x31027e0, "Movq");
  cpu.unimplemented(0x31027e3, "Movq");
  cpu.unimplemented(0x31027e6, "Movq");
  cpu.unimplemented(0x31027e9, "Psrlq");
  cpu.unimplemented(0x31027ed, "Psllq");
  cpu.unimplemented(0x31027f1, "Pand");
  cpu.unimplemented(0x31027f8, "Por");
  cpu.unimplemented(0x31027fb, "Psrlq");
  cpu.unimplemented(0x31027ff, "Pand");
  cpu.unimplemented(0x3102806, "Psrlq");
  cpu.unimplemented(0x310280a, "Paddw");
  cpu.unimplemented(0x310280d, "Movq");
  cpu.unimplemented(0x3102810, "Punpcklwd");
  cpu.unimplemented(0x3102813, "Punpckhwd");
  cpu.unimplemented(0x3102816, "Movq");
  cpu.unimplemented(0x3102819, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102823, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31027dc; continue; }
  pc = 0x3102829; continue;
 }
 0x3102829 => {
  cpu.unimplemented(0x3102829, "Punpcklwd");
  cpu.unimplemented(0x310282c, "Punpckhwd");
  cpu.unimplemented(0x310282f, "Movq");
  cpu.unimplemented(0x3102832, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x310284a, "Movq");
  cpu.unimplemented(0x3102851, "Movq");
  pc = 0x3102854; continue;
 }
 0x3102854 => {
  cpu.unimplemented(0x3102854, "Movq");
  cpu.unimplemented(0x3102858, "Movq");
  cpu.unimplemented(0x310285b, "Movq");
  cpu.unimplemented(0x310285e, "Movq");
  cpu.unimplemented(0x3102861, "Psrlq");
  cpu.unimplemented(0x3102865, "Psllq");
  cpu.unimplemented(0x3102869, "Pand");
  cpu.unimplemented(0x3102870, "Por");
  cpu.unimplemented(0x3102873, "Psrlq");
  cpu.unimplemented(0x3102877, "Pand");
  cpu.unimplemented(0x310287e, "Psrlq");
  cpu.unimplemented(0x3102882, "Paddw");
  cpu.unimplemented(0x3102885, "Movq");
  cpu.unimplemented(0x3102888, "Punpcklwd");
  cpu.unimplemented(0x310288b, "Punpckhwd");
  cpu.unimplemented(0x310288e, "Movq");
  cpu.unimplemented(0x3102891, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x310289b, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102854; continue; }
  pc = 0x31028a1; continue;
 }
 0x31028a1 => {
  cpu.unimplemented(0x31028a1, "Punpcklwd");
  cpu.unimplemented(0x31028a4, "Punpckhwd");
  cpu.unimplemented(0x31028a7, "Movq");
  cpu.unimplemented(0x31028aa, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x31028b9; continue;
 }
 0x31028b9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31028c1, "Movq");
  cpu.unimplemented(0x31028c8, "Movq");
  pc = 0x31028cb; continue;
 }
 0x31028cb => {
  cpu.unimplemented(0x31028cb, "Movq");
  cpu.unimplemented(0x31028cf, "Movq");
  cpu.unimplemented(0x31028d2, "Movq");
  cpu.unimplemented(0x31028d5, "Movq");
  cpu.unimplemented(0x31028d8, "Psrlq");
  cpu.unimplemented(0x31028dc, "Psllq");
  cpu.unimplemented(0x31028e0, "Pand");
  cpu.unimplemented(0x31028e7, "Por");
  cpu.unimplemented(0x31028ea, "Psrlq");
  cpu.unimplemented(0x31028ee, "Pand");
  cpu.unimplemented(0x31028f5, "Psrlq");
  cpu.unimplemented(0x31028f9, "Paddw");
  cpu.unimplemented(0x31028fc, "Movq");
  cpu.unimplemented(0x31028ff, "Punpcklwd");
  cpu.unimplemented(0x3102902, "Punpckhwd");
  cpu.unimplemented(0x3102905, "Movq");
  cpu.unimplemented(0x3102908, "Pand");
  cpu.unimplemented(0x310290b, "Movq");
  cpu.unimplemented(0x3102912, "Movq");
  cpu.unimplemented(0x3102916, "Pand");
  cpu.unimplemented(0x3102919, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102926, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31028cb; continue; }
  pc = 0x310292c; continue;
 }
 0x310292c => {
  cpu.unimplemented(0x310292c, "Punpcklwd");
  cpu.unimplemented(0x310292f, "Punpckhwd");
  cpu.unimplemented(0x3102932, "Movq");
  cpu.unimplemented(0x3102935, "Pand");
  cpu.unimplemented(0x3102938, "Movq");
  cpu.unimplemented(0x310293f, "Movq");
  cpu.unimplemented(0x3102943, "Pand");
  cpu.unimplemented(0x3102946, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31028b9; continue; }
  pc = 0x3102960; continue;
 }
 0x3102960 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102968, "Movq");
  cpu.unimplemented(0x310296f, "Movq");
  pc = 0x3102972; continue;
 }
 0x3102972 => {
  cpu.unimplemented(0x3102972, "Movq");
  cpu.unimplemented(0x3102976, "Movq");
  cpu.unimplemented(0x3102979, "Movq");
  cpu.unimplemented(0x310297c, "Movq");
  cpu.unimplemented(0x310297f, "Psrlq");
  cpu.unimplemented(0x3102983, "Psllq");
  cpu.unimplemented(0x3102987, "Pand");
  cpu.unimplemented(0x310298e, "Por");
  cpu.unimplemented(0x3102991, "Psrlq");
  cpu.unimplemented(0x3102995, "Pand");
  cpu.unimplemented(0x310299c, "Psrlq");
  cpu.unimplemented(0x31029a0, "Paddw");
  cpu.unimplemented(0x31029a3, "Movq");
  cpu.unimplemented(0x31029a6, "Punpcklwd");
  cpu.unimplemented(0x31029a9, "Punpckhwd");
  cpu.unimplemented(0x31029ac, "Movq");
  cpu.unimplemented(0x31029af, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31029b9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102972; continue; }
  pc = 0x31029bf; continue;
 }
 0x31029bf => {
  cpu.unimplemented(0x31029bf, "Punpcklwd");
  cpu.unimplemented(0x31029c2, "Punpckhwd");
  cpu.unimplemented(0x31029c5, "Movq");
  cpu.unimplemented(0x31029c8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31029e0, "Movq");
  cpu.unimplemented(0x31029e7, "Movq");
  pc = 0x31029ea; continue;
 }
 0x31029ea => {
  cpu.unimplemented(0x31029ea, "Movq");
  cpu.unimplemented(0x31029ee, "Movq");
  cpu.unimplemented(0x31029f1, "Movq");
  cpu.unimplemented(0x31029f4, "Movq");
  cpu.unimplemented(0x31029f7, "Psrlq");
  cpu.unimplemented(0x31029fb, "Psllq");
  cpu.unimplemented(0x31029ff, "Pand");
  cpu.unimplemented(0x3102a06, "Por");
  cpu.unimplemented(0x3102a09, "Psrlq");
  cpu.unimplemented(0x3102a0d, "Pand");
  cpu.unimplemented(0x3102a14, "Psrlq");
  cpu.unimplemented(0x3102a18, "Paddw");
  cpu.unimplemented(0x3102a1b, "Movq");
  cpu.unimplemented(0x3102a1e, "Punpcklwd");
  cpu.unimplemented(0x3102a21, "Punpckhwd");
  cpu.unimplemented(0x3102a24, "Movq");
  cpu.unimplemented(0x3102a27, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102a31, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31029ea; continue; }
  pc = 0x3102a37; continue;
 }
 0x3102a37 => {
  cpu.unimplemented(0x3102a37, "Punpcklwd");
  cpu.unimplemented(0x3102a3a, "Punpckhwd");
  cpu.unimplemented(0x3102a3d, "Movq");
  cpu.unimplemented(0x3102a40, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3102a9f; continue; }
  pc = 0x3102a61; continue;
 }
 0x3102a61 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3102a96; continue; }
  pc = 0x3102a85; continue;
 }
 0x3102a85 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3102a96; continue;
 }
 0x3102a96 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3102a9f; continue;
 }
 0x3102a9f => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102aad, "Movq");
  cpu.unimplemented(0x3102ab4, "Movq");
  pc = 0x3102ab7; continue;
 }
 0x3102ab7 => {
  cpu.unimplemented(0x3102ab7, "Movq");
  cpu.unimplemented(0x3102abb, "Movq");
  cpu.unimplemented(0x3102abe, "Movq");
  cpu.unimplemented(0x3102ac1, "Movq");
  cpu.unimplemented(0x3102ac4, "Psrlq");
  cpu.unimplemented(0x3102ac8, "Psllq");
  cpu.unimplemented(0x3102acc, "Pand");
  cpu.unimplemented(0x3102ad3, "Por");
  cpu.unimplemented(0x3102ad6, "Psrlq");
  cpu.unimplemented(0x3102ada, "Pand");
  cpu.unimplemented(0x3102ae1, "Psrlq");
  cpu.unimplemented(0x3102ae5, "Paddw");
  cpu.unimplemented(0x3102ae8, "Movq");
  cpu.unimplemented(0x3102aeb, "Punpcklwd");
  cpu.unimplemented(0x3102aee, "Punpckhwd");
  cpu.unimplemented(0x3102af1, "Movq");
  cpu.unimplemented(0x3102af4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102afe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102ab7; continue; }
  pc = 0x3102b04; continue;
 }
 0x3102b04 => {
  cpu.unimplemented(0x3102b04, "Punpcklwd");
  cpu.unimplemented(0x3102b07, "Punpckhwd");
  cpu.unimplemented(0x3102b0a, "Movq");
  cpu.unimplemented(0x3102b0d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3102b1c; continue;
 }
 0x3102b1c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102b24, "Movq");
  cpu.unimplemented(0x3102b2b, "Movq");
  pc = 0x3102b2e; continue;
 }
 0x3102b2e => {
  cpu.unimplemented(0x3102b2e, "Movq");
  cpu.unimplemented(0x3102b32, "Movq");
  cpu.unimplemented(0x3102b35, "Movq");
  cpu.unimplemented(0x3102b38, "Movq");
  cpu.unimplemented(0x3102b3b, "Psrlq");
  cpu.unimplemented(0x3102b3f, "Psllq");
  cpu.unimplemented(0x3102b43, "Pand");
  cpu.unimplemented(0x3102b4a, "Por");
  cpu.unimplemented(0x3102b4d, "Psrlq");
  cpu.unimplemented(0x3102b51, "Pand");
  cpu.unimplemented(0x3102b58, "Psrlq");
  cpu.unimplemented(0x3102b5c, "Paddw");
  cpu.unimplemented(0x3102b5f, "Movq");
  cpu.unimplemented(0x3102b62, "Punpcklwd");
  cpu.unimplemented(0x3102b65, "Punpckhwd");
  cpu.unimplemented(0x3102b68, "Movq");
  cpu.unimplemented(0x3102b6b, "Pand");
  cpu.unimplemented(0x3102b6e, "Movq");
  cpu.unimplemented(0x3102b75, "Movq");
  cpu.unimplemented(0x3102b79, "Pand");
  cpu.unimplemented(0x3102b7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102b89, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102b2e; continue; }
  pc = 0x3102b8f; continue;
 }
 0x3102b8f => {
  cpu.unimplemented(0x3102b8f, "Punpcklwd");
  cpu.unimplemented(0x3102b92, "Punpckhwd");
  cpu.unimplemented(0x3102b95, "Movq");
  cpu.unimplemented(0x3102b98, "Pand");
  cpu.unimplemented(0x3102b9b, "Movq");
  cpu.unimplemented(0x3102ba2, "Movq");
  cpu.unimplemented(0x3102ba6, "Pand");
  cpu.unimplemented(0x3102ba9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102b1c; continue; }
  pc = 0x3102bc3; continue;
 }
 0x3102bc3 => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102bcb, "Movq");
  cpu.unimplemented(0x3102bd2, "Movq");
  pc = 0x3102bd5; continue;
 }
 0x3102bd5 => {
  cpu.unimplemented(0x3102bd5, "Movq");
  cpu.unimplemented(0x3102bd9, "Movq");
  cpu.unimplemented(0x3102bdc, "Movq");
  cpu.unimplemented(0x3102bdf, "Movq");
  cpu.unimplemented(0x3102be2, "Psrlq");
  cpu.unimplemented(0x3102be6, "Psllq");
  cpu.unimplemented(0x3102bea, "Pand");
  cpu.unimplemented(0x3102bf1, "Por");
  cpu.unimplemented(0x3102bf4, "Psrlq");
  cpu.unimplemented(0x3102bf8, "Pand");
  cpu.unimplemented(0x3102bff, "Psrlq");
  cpu.unimplemented(0x3102c03, "Paddw");
  cpu.unimplemented(0x3102c06, "Movq");
  cpu.unimplemented(0x3102c09, "Punpcklwd");
  cpu.unimplemented(0x3102c0c, "Punpckhwd");
  cpu.unimplemented(0x3102c0f, "Movq");
  cpu.unimplemented(0x3102c12, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102c1c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102bd5; continue; }
  pc = 0x3102c22; continue;
 }
 0x3102c22 => {
  cpu.unimplemented(0x3102c22, "Punpcklwd");
  cpu.unimplemented(0x3102c25, "Punpckhwd");
  cpu.unimplemented(0x3102c28, "Movq");
  cpu.unimplemented(0x3102c2b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3102c8a; continue; }
  pc = 0x3102c4c; continue;
 }
 0x3102c4c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3102c81; continue; }
  pc = 0x3102c70; continue;
 }
 0x3102c70 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3102c81; continue;
 }
 0x3102c81 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3102c8a; continue;
 }
 0x3102c8a => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102c98, "Movq");
  cpu.unimplemented(0x3102c9f, "Movq");
  pc = 0x3102ca2; continue;
 }
 0x3102ca2 => {
  cpu.unimplemented(0x3102ca2, "Movq");
  cpu.unimplemented(0x3102ca6, "Movq");
  cpu.unimplemented(0x3102ca9, "Movq");
  cpu.unimplemented(0x3102cac, "Movq");
  cpu.unimplemented(0x3102caf, "Psrlq");
  cpu.unimplemented(0x3102cb3, "Psllq");
  cpu.unimplemented(0x3102cb7, "Pand");
  cpu.unimplemented(0x3102cbe, "Por");
  cpu.unimplemented(0x3102cc1, "Psrlq");
  cpu.unimplemented(0x3102cc5, "Pand");
  cpu.unimplemented(0x3102ccc, "Psrlq");
  cpu.unimplemented(0x3102cd0, "Paddw");
  cpu.unimplemented(0x3102cd3, "Movq");
  cpu.unimplemented(0x3102cd6, "Punpcklwd");
  cpu.unimplemented(0x3102cd9, "Punpckhwd");
  cpu.unimplemented(0x3102cdc, "Movq");
  cpu.unimplemented(0x3102cdf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102ce9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102ca2; continue; }
  pc = 0x3102cef; continue;
 }
 0x3102cef => {
  cpu.unimplemented(0x3102cef, "Punpcklwd");
  cpu.unimplemented(0x3102cf2, "Punpckhwd");
  cpu.unimplemented(0x3102cf5, "Movq");
  cpu.unimplemented(0x3102cf8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102d10, "Movq");
  cpu.unimplemented(0x3102d17, "Movq");
  pc = 0x3102d1a; continue;
 }
 0x3102d1a => {
  cpu.unimplemented(0x3102d1a, "Movq");
  cpu.unimplemented(0x3102d1e, "Movq");
  cpu.unimplemented(0x3102d21, "Movq");
  cpu.unimplemented(0x3102d24, "Movq");
  cpu.unimplemented(0x3102d27, "Psrlq");
  cpu.unimplemented(0x3102d2b, "Psllq");
  cpu.unimplemented(0x3102d2f, "Pand");
  cpu.unimplemented(0x3102d36, "Por");
  cpu.unimplemented(0x3102d39, "Psrlq");
  cpu.unimplemented(0x3102d3d, "Pand");
  cpu.unimplemented(0x3102d44, "Psrlq");
  cpu.unimplemented(0x3102d48, "Paddw");
  cpu.unimplemented(0x3102d4b, "Movq");
  cpu.unimplemented(0x3102d4e, "Punpcklwd");
  cpu.unimplemented(0x3102d51, "Punpckhwd");
  cpu.unimplemented(0x3102d54, "Movq");
  cpu.unimplemented(0x3102d57, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102d61, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102d1a; continue; }
  pc = 0x3102d67; continue;
 }
 0x3102d67 => {
  cpu.unimplemented(0x3102d67, "Punpcklwd");
  cpu.unimplemented(0x3102d6a, "Punpckhwd");
  cpu.unimplemented(0x3102d6d, "Movq");
  cpu.unimplemented(0x3102d70, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3102d7f; continue;
 }
 0x3102d7f => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102d87, "Movq");
  cpu.unimplemented(0x3102d8e, "Movq");
  pc = 0x3102d91; continue;
 }
 0x3102d91 => {
  cpu.unimplemented(0x3102d91, "Movq");
  cpu.unimplemented(0x3102d95, "Movq");
  cpu.unimplemented(0x3102d98, "Movq");
  cpu.unimplemented(0x3102d9b, "Movq");
  cpu.unimplemented(0x3102d9e, "Psrlq");
  cpu.unimplemented(0x3102da2, "Psllq");
  cpu.unimplemented(0x3102da6, "Pand");
  cpu.unimplemented(0x3102dad, "Por");
  cpu.unimplemented(0x3102db0, "Psrlq");
  cpu.unimplemented(0x3102db4, "Pand");
  cpu.unimplemented(0x3102dbb, "Psrlq");
  cpu.unimplemented(0x3102dbf, "Paddw");
  cpu.unimplemented(0x3102dc2, "Movq");
  cpu.unimplemented(0x3102dc5, "Punpcklwd");
  cpu.unimplemented(0x3102dc8, "Punpckhwd");
  cpu.unimplemented(0x3102dcb, "Movq");
  cpu.unimplemented(0x3102dce, "Pand");
  cpu.unimplemented(0x3102dd1, "Movq");
  cpu.unimplemented(0x3102dd8, "Movq");
  cpu.unimplemented(0x3102ddc, "Pand");
  cpu.unimplemented(0x3102ddf, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102dec, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102d91; continue; }
  pc = 0x3102df2; continue;
 }
 0x3102df2 => {
  cpu.unimplemented(0x3102df2, "Punpcklwd");
  cpu.unimplemented(0x3102df5, "Punpckhwd");
  cpu.unimplemented(0x3102df8, "Movq");
  cpu.unimplemented(0x3102dfb, "Pand");
  cpu.unimplemented(0x3102dfe, "Movq");
  cpu.unimplemented(0x3102e05, "Movq");
  cpu.unimplemented(0x3102e09, "Pand");
  cpu.unimplemented(0x3102e0c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102d7f; continue; }
  pc = 0x3102e26; continue;
 }
 0x3102e26 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3102e7b; continue; }
  pc = 0x3102e3d; continue;
 }
 0x3102e3d => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3102e72; continue; }
  pc = 0x3102e61; continue;
 }
 0x3102e61 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3102e72; continue;
 }
 0x3102e72 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3102e7b; continue;
 }
 0x3102e7b => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x3102e86; continue;
 }
 0x3102e86 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102e8e, "Movq");
  cpu.unimplemented(0x3102e95, "Movq");
  pc = 0x3102e98; continue;
 }
 0x3102e98 => {
  cpu.unimplemented(0x3102e98, "Movq");
  cpu.unimplemented(0x3102e9c, "Movq");
  cpu.unimplemented(0x3102e9f, "Movq");
  cpu.unimplemented(0x3102ea2, "Movq");
  cpu.unimplemented(0x3102ea5, "Psrlq");
  cpu.unimplemented(0x3102ea9, "Psllq");
  cpu.unimplemented(0x3102ead, "Pand");
  cpu.unimplemented(0x3102eb4, "Por");
  cpu.unimplemented(0x3102eb7, "Psrlq");
  cpu.unimplemented(0x3102ebb, "Pand");
  cpu.unimplemented(0x3102ec2, "Psrlq");
  cpu.unimplemented(0x3102ec6, "Paddw");
  cpu.unimplemented(0x3102ec9, "Movq");
  cpu.unimplemented(0x3102ecc, "Punpcklwd");
  cpu.unimplemented(0x3102ecf, "Punpckhwd");
  cpu.unimplemented(0x3102ed2, "Movq");
  cpu.unimplemented(0x3102ed5, "Pand");
  cpu.unimplemented(0x3102ed8, "Movq");
  cpu.unimplemented(0x3102edf, "Movq");
  cpu.unimplemented(0x3102ee3, "Pand");
  cpu.unimplemented(0x3102ee6, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102ef3, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102e98; continue; }
  pc = 0x3102ef9; continue;
 }
 0x3102ef9 => {
  cpu.unimplemented(0x3102ef9, "Punpcklwd");
  cpu.unimplemented(0x3102efc, "Punpckhwd");
  cpu.unimplemented(0x3102eff, "Movq");
  cpu.unimplemented(0x3102f02, "Pand");
  cpu.unimplemented(0x3102f05, "Movq");
  cpu.unimplemented(0x3102f0c, "Movq");
  cpu.unimplemented(0x3102f10, "Pand");
  cpu.unimplemented(0x3102f13, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3102e86; continue; }
  pc = 0x3102f2d; continue;
 }
 0x3102f2d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102f35, "Movq");
  cpu.unimplemented(0x3102f3c, "Movq");
  pc = 0x3102f3f; continue;
 }
 0x3102f3f => {
  cpu.unimplemented(0x3102f3f, "Movq");
  cpu.unimplemented(0x3102f43, "Movq");
  cpu.unimplemented(0x3102f46, "Movq");
  cpu.unimplemented(0x3102f49, "Movq");
  cpu.unimplemented(0x3102f4c, "Psrlq");
  cpu.unimplemented(0x3102f50, "Psllq");
  cpu.unimplemented(0x3102f54, "Pand");
  cpu.unimplemented(0x3102f5b, "Por");
  cpu.unimplemented(0x3102f5e, "Psrlq");
  cpu.unimplemented(0x3102f62, "Pand");
  cpu.unimplemented(0x3102f69, "Psrlq");
  cpu.unimplemented(0x3102f6d, "Paddw");
  cpu.unimplemented(0x3102f70, "Movq");
  cpu.unimplemented(0x3102f73, "Punpcklwd");
  cpu.unimplemented(0x3102f76, "Punpckhwd");
  cpu.unimplemented(0x3102f79, "Movq");
  cpu.unimplemented(0x3102f7c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102f86, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102f3f; continue; }
  pc = 0x3102f8c; continue;
 }
 0x3102f8c => {
  cpu.unimplemented(0x3102f8c, "Punpcklwd");
  cpu.unimplemented(0x3102f8f, "Punpckhwd");
  cpu.unimplemented(0x3102f92, "Movq");
  cpu.unimplemented(0x3102f95, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3102fad, "Movq");
  cpu.unimplemented(0x3102fb4, "Movq");
  pc = 0x3102fb7; continue;
 }
 0x3102fb7 => {
  cpu.unimplemented(0x3102fb7, "Movq");
  cpu.unimplemented(0x3102fbb, "Movq");
  cpu.unimplemented(0x3102fbe, "Movq");
  cpu.unimplemented(0x3102fc1, "Movq");
  cpu.unimplemented(0x3102fc4, "Psrlq");
  cpu.unimplemented(0x3102fc8, "Psllq");
  cpu.unimplemented(0x3102fcc, "Pand");
  cpu.unimplemented(0x3102fd3, "Por");
  cpu.unimplemented(0x3102fd6, "Psrlq");
  cpu.unimplemented(0x3102fda, "Pand");
  cpu.unimplemented(0x3102fe1, "Psrlq");
  cpu.unimplemented(0x3102fe5, "Paddw");
  cpu.unimplemented(0x3102fe8, "Movq");
  cpu.unimplemented(0x3102feb, "Punpcklwd");
  cpu.unimplemented(0x3102fee, "Punpckhwd");
  cpu.unimplemented(0x3102ff1, "Movq");
  cpu.unimplemented(0x3102ff4, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3102ffe, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3102fb7; continue; }
  pc = 0x3103004; continue;
 }
 0x3103004 => {
  cpu.unimplemented(0x3103004, "Punpcklwd");
  cpu.unimplemented(0x3103007, "Punpckhwd");
  cpu.unimplemented(0x310300a, "Movq");
  cpu.unimplemented(0x310300d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310306c; continue; }
  pc = 0x310302e; continue;
 }
 0x310302e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3103063; continue; }
  pc = 0x3103052; continue;
 }
 0x3103052 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3103063; continue;
 }
 0x3103063 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310306c; continue;
 }
 0x310306c => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x310307a, "Movq");
  cpu.unimplemented(0x3103081, "Movq");
  pc = 0x3103084; continue;
 }
 0x3103084 => {
  cpu.unimplemented(0x3103084, "Movq");
  cpu.unimplemented(0x3103088, "Movq");
  cpu.unimplemented(0x310308b, "Movq");
  cpu.unimplemented(0x310308e, "Movq");
  cpu.unimplemented(0x3103091, "Psrlq");
  cpu.unimplemented(0x3103095, "Psllq");
  cpu.unimplemented(0x3103099, "Pand");
  cpu.unimplemented(0x31030a0, "Por");
  cpu.unimplemented(0x31030a3, "Psrlq");
  cpu.unimplemented(0x31030a7, "Pand");
  cpu.unimplemented(0x31030ae, "Psrlq");
  cpu.unimplemented(0x31030b2, "Paddw");
  cpu.unimplemented(0x31030b5, "Movq");
  cpu.unimplemented(0x31030b8, "Punpcklwd");
  cpu.unimplemented(0x31030bb, "Punpckhwd");
  cpu.unimplemented(0x31030be, "Movq");
  cpu.unimplemented(0x31030c1, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31030cb, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3103084; continue; }
  pc = 0x31030d1; continue;
 }
 0x31030d1 => {
  cpu.unimplemented(0x31030d1, "Punpcklwd");
  cpu.unimplemented(0x31030d4, "Punpckhwd");
  cpu.unimplemented(0x31030d7, "Movq");
  cpu.unimplemented(0x31030da, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0x19u32; cpu.edx = v; }
  pc = 0x31030e9; continue;
 }
 0x31030e9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31030f1, "Movq");
  cpu.unimplemented(0x31030f8, "Movq");
  pc = 0x31030fb; continue;
 }
 0x31030fb => {
  cpu.unimplemented(0x31030fb, "Movq");
  cpu.unimplemented(0x31030ff, "Movq");
  cpu.unimplemented(0x3103102, "Movq");
  cpu.unimplemented(0x3103105, "Movq");
  cpu.unimplemented(0x3103108, "Psrlq");
  cpu.unimplemented(0x310310c, "Psllq");
  cpu.unimplemented(0x3103110, "Pand");
  cpu.unimplemented(0x3103117, "Por");
  cpu.unimplemented(0x310311a, "Psrlq");
  cpu.unimplemented(0x310311e, "Pand");
  cpu.unimplemented(0x3103125, "Psrlq");
  cpu.unimplemented(0x3103129, "Paddw");
  cpu.unimplemented(0x310312c, "Movq");
  cpu.unimplemented(0x310312f, "Punpcklwd");
  cpu.unimplemented(0x3103132, "Punpckhwd");
  cpu.unimplemented(0x3103135, "Movq");
  cpu.unimplemented(0x3103138, "Pand");
  cpu.unimplemented(0x310313b, "Movq");
  cpu.unimplemented(0x3103142, "Movq");
  cpu.unimplemented(0x3103146, "Pand");
  cpu.unimplemented(0x3103149, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x3103156, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31030fb; continue; }
  pc = 0x310315c; continue;
 }
 0x310315c => {
  cpu.unimplemented(0x310315c, "Punpcklwd");
  cpu.unimplemented(0x310315f, "Punpckhwd");
  cpu.unimplemented(0x3103162, "Movq");
  cpu.unimplemented(0x3103165, "Pand");
  cpu.unimplemented(0x3103168, "Movq");
  cpu.unimplemented(0x310316f, "Movq");
  cpu.unimplemented(0x3103173, "Pand");
  cpu.unimplemented(0x3103176, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31030e9; continue; }
  pc = 0x3103190; continue;
 }
 0x3103190 => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103198, "Movq");
  cpu.unimplemented(0x310319f, "Movq");
  pc = 0x31031a2; continue;
 }
 0x31031a2 => {
  cpu.unimplemented(0x31031a2, "Movq");
  cpu.unimplemented(0x31031a6, "Movq");
  cpu.unimplemented(0x31031a9, "Movq");
  cpu.unimplemented(0x31031ac, "Movq");
  cpu.unimplemented(0x31031af, "Psrlq");
  cpu.unimplemented(0x31031b3, "Psllq");
  cpu.unimplemented(0x31031b7, "Pand");
  cpu.unimplemented(0x31031be, "Por");
  cpu.unimplemented(0x31031c1, "Psrlq");
  cpu.unimplemented(0x31031c5, "Pand");
  cpu.unimplemented(0x31031cc, "Psrlq");
  cpu.unimplemented(0x31031d0, "Paddw");
  cpu.unimplemented(0x31031d3, "Movq");
  cpu.unimplemented(0x31031d6, "Punpcklwd");
  cpu.unimplemented(0x31031d9, "Punpckhwd");
  cpu.unimplemented(0x31031dc, "Movq");
  cpu.unimplemented(0x31031df, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31031e9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31031a2; continue; }
  pc = 0x31031ef; continue;
 }
 0x31031ef => {
  cpu.unimplemented(0x31031ef, "Punpcklwd");
  cpu.unimplemented(0x31031f2, "Punpckhwd");
  cpu.unimplemented(0x31031f5, "Movq");
  cpu.unimplemented(0x31031f8, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3103257; continue; }
  pc = 0x3103219; continue;
 }
 0x3103219 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310324e; continue; }
  pc = 0x310323d; continue;
 }
 0x310323d => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310324e; continue;
 }
 0x310324e => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3103257; continue;
 }
 0x3103257 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103265, "Movq");
  cpu.unimplemented(0x310326c, "Movq");
  pc = 0x310326f; continue;
 }
 0x310326f => {
  cpu.unimplemented(0x310326f, "Movq");
  cpu.unimplemented(0x3103273, "Movq");
  cpu.unimplemented(0x3103276, "Movq");
  cpu.unimplemented(0x3103279, "Movq");
  cpu.unimplemented(0x310327c, "Psrlq");
  cpu.unimplemented(0x3103280, "Psllq");
  cpu.unimplemented(0x3103284, "Pand");
  cpu.unimplemented(0x310328b, "Por");
  cpu.unimplemented(0x310328e, "Psrlq");
  cpu.unimplemented(0x3103292, "Pand");
  cpu.unimplemented(0x3103299, "Psrlq");
  cpu.unimplemented(0x310329d, "Paddw");
  cpu.unimplemented(0x31032a0, "Movq");
  cpu.unimplemented(0x31032a3, "Punpcklwd");
  cpu.unimplemented(0x31032a6, "Punpckhwd");
  cpu.unimplemented(0x31032a9, "Movq");
  cpu.unimplemented(0x31032ac, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31032b6, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x310326f; continue; }
  pc = 0x31032bc; continue;
 }
 0x31032bc => {
  cpu.unimplemented(0x31032bc, "Punpcklwd");
  cpu.unimplemented(0x31032bf, "Punpckhwd");
  cpu.unimplemented(0x31032c2, "Movq");
  cpu.unimplemented(0x31032c5, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31032dd, "Movq");
  cpu.unimplemented(0x31032e4, "Movq");
  pc = 0x31032e7; continue;
 }
 0x31032e7 => {
  cpu.unimplemented(0x31032e7, "Movq");
  cpu.unimplemented(0x31032eb, "Movq");
  cpu.unimplemented(0x31032ee, "Movq");
  cpu.unimplemented(0x31032f1, "Movq");
  cpu.unimplemented(0x31032f4, "Psrlq");
  cpu.unimplemented(0x31032f8, "Psllq");
  cpu.unimplemented(0x31032fc, "Pand");
  cpu.unimplemented(0x3103303, "Por");
  cpu.unimplemented(0x3103306, "Psrlq");
  cpu.unimplemented(0x310330a, "Pand");
  cpu.unimplemented(0x3103311, "Psrlq");
  cpu.unimplemented(0x3103315, "Paddw");
  cpu.unimplemented(0x3103318, "Movq");
  cpu.unimplemented(0x310331b, "Punpcklwd");
  cpu.unimplemented(0x310331e, "Punpckhwd");
  cpu.unimplemented(0x3103321, "Movq");
  cpu.unimplemented(0x3103324, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x310332e, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31032e7; continue; }
  pc = 0x3103334; continue;
 }
 0x3103334 => {
  cpu.unimplemented(0x3103334, "Punpcklwd");
  cpu.unimplemented(0x3103337, "Punpckhwd");
  cpu.unimplemented(0x310333a, "Movq");
  cpu.unimplemented(0x310333d, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x310366b; continue; }
  pc = 0x3103357; continue;
 }
 0x3103357 => {
  { let v: u32 = 0x18u32; cpu.edx = v; }
  pc = 0x310335c; continue;
 }
 0x310335c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103364, "Movq");
  cpu.unimplemented(0x310336b, "Movq");
  pc = 0x310336e; continue;
 }
 0x310336e => {
  cpu.unimplemented(0x310336e, "Movq");
  cpu.unimplemented(0x3103372, "Movq");
  cpu.unimplemented(0x3103375, "Movq");
  cpu.unimplemented(0x3103378, "Movq");
  cpu.unimplemented(0x310337b, "Psrlq");
  cpu.unimplemented(0x310337f, "Psllq");
  cpu.unimplemented(0x3103383, "Pand");
  cpu.unimplemented(0x310338a, "Por");
  cpu.unimplemented(0x310338d, "Psrlq");
  cpu.unimplemented(0x3103391, "Pand");
  cpu.unimplemented(0x3103398, "Psrlq");
  cpu.unimplemented(0x310339c, "Paddw");
  cpu.unimplemented(0x310339f, "Movq");
  cpu.unimplemented(0x31033a2, "Punpcklwd");
  cpu.unimplemented(0x31033a5, "Punpckhwd");
  cpu.unimplemented(0x31033a8, "Movq");
  cpu.unimplemented(0x31033ab, "Pand");
  cpu.unimplemented(0x31033ae, "Movq");
  cpu.unimplemented(0x31033b5, "Movq");
  cpu.unimplemented(0x31033b9, "Pand");
  cpu.unimplemented(0x31033bc, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31033c9, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x310336e; continue; }
  pc = 0x31033cf; continue;
 }
 0x31033cf => {
  cpu.unimplemented(0x31033cf, "Punpcklwd");
  cpu.unimplemented(0x31033d2, "Punpckhwd");
  cpu.unimplemented(0x31033d5, "Movq");
  cpu.unimplemented(0x31033d8, "Pand");
  cpu.unimplemented(0x31033db, "Movq");
  cpu.unimplemented(0x31033e2, "Movq");
  cpu.unimplemented(0x31033e6, "Pand");
  cpu.unimplemented(0x31033e9, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310335c; continue; }
  pc = 0x3103403; continue;
 }
 0x3103403 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x310340b, "Movq");
  cpu.unimplemented(0x3103412, "Movq");
  pc = 0x3103415; continue;
 }
 0x3103415 => {
  cpu.unimplemented(0x3103415, "Movq");
  cpu.unimplemented(0x3103419, "Movq");
  cpu.unimplemented(0x310341c, "Movq");
  cpu.unimplemented(0x310341f, "Movq");
  cpu.unimplemented(0x3103422, "Psrlq");
  cpu.unimplemented(0x3103426, "Psllq");
  cpu.unimplemented(0x310342a, "Pand");
  cpu.unimplemented(0x3103431, "Por");
  cpu.unimplemented(0x3103434, "Psrlq");
  cpu.unimplemented(0x3103438, "Pand");
  cpu.unimplemented(0x310343f, "Psrlq");
  cpu.unimplemented(0x3103443, "Paddw");
  cpu.unimplemented(0x3103446, "Movq");
  cpu.unimplemented(0x3103449, "Punpcklwd");
  cpu.unimplemented(0x310344c, "Punpckhwd");
  cpu.unimplemented(0x310344f, "Movq");
  cpu.unimplemented(0x3103452, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x310345c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3103415; continue; }
  pc = 0x3103462; continue;
 }
 0x3103462 => {
  cpu.unimplemented(0x3103462, "Punpcklwd");
  cpu.unimplemented(0x3103465, "Punpckhwd");
  cpu.unimplemented(0x3103468, "Movq");
  cpu.unimplemented(0x310346b, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103483, "Movq");
  cpu.unimplemented(0x310348a, "Movq");
  pc = 0x310348d; continue;
 }
 0x310348d => {
  cpu.unimplemented(0x310348d, "Movq");
  cpu.unimplemented(0x3103491, "Movq");
  cpu.unimplemented(0x3103494, "Movq");
  cpu.unimplemented(0x3103497, "Movq");
  cpu.unimplemented(0x310349a, "Psrlq");
  cpu.unimplemented(0x310349e, "Psllq");
  cpu.unimplemented(0x31034a2, "Pand");
  cpu.unimplemented(0x31034a9, "Por");
  cpu.unimplemented(0x31034ac, "Psrlq");
  cpu.unimplemented(0x31034b0, "Pand");
  cpu.unimplemented(0x31034b7, "Psrlq");
  cpu.unimplemented(0x31034bb, "Paddw");
  cpu.unimplemented(0x31034be, "Movq");
  cpu.unimplemented(0x31034c1, "Punpcklwd");
  cpu.unimplemented(0x31034c4, "Punpckhwd");
  cpu.unimplemented(0x31034c7, "Movq");
  cpu.unimplemented(0x31034ca, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31034d4, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x310348d; continue; }
  pc = 0x31034da; continue;
 }
 0x31034da => {
  cpu.unimplemented(0x31034da, "Punpcklwd");
  cpu.unimplemented(0x31034dd, "Punpckhwd");
  cpu.unimplemented(0x31034e0, "Movq");
  cpu.unimplemented(0x31034e3, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3103542; continue; }
  pc = 0x3103504; continue;
 }
 0x3103504 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3103539; continue; }
  pc = 0x3103528; continue;
 }
 0x3103528 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3103539; continue;
 }
 0x3103539 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3103542; continue;
 }
 0x3103542 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103550, "Movq");
  cpu.unimplemented(0x3103557, "Movq");
  pc = 0x310355a; continue;
 }
 0x310355a => {
  cpu.unimplemented(0x310355a, "Movq");
  cpu.unimplemented(0x310355e, "Movq");
  cpu.unimplemented(0x3103561, "Movq");
  cpu.unimplemented(0x3103564, "Movq");
  cpu.unimplemented(0x3103567, "Psrlq");
  cpu.unimplemented(0x310356b, "Psllq");
  cpu.unimplemented(0x310356f, "Pand");
  cpu.unimplemented(0x3103576, "Por");
  cpu.unimplemented(0x3103579, "Psrlq");
  cpu.unimplemented(0x310357d, "Pand");
  cpu.unimplemented(0x3103584, "Psrlq");
  cpu.unimplemented(0x3103588, "Paddw");
  cpu.unimplemented(0x310358b, "Movq");
  cpu.unimplemented(0x310358e, "Punpcklwd");
  cpu.unimplemented(0x3103591, "Punpckhwd");
  cpu.unimplemented(0x3103594, "Movq");
  cpu.unimplemented(0x3103597, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31035a1, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x310355a; continue; }
  pc = 0x31035a7; continue;
 }
 0x31035a7 => {
  cpu.unimplemented(0x31035a7, "Punpcklwd");
  cpu.unimplemented(0x31035aa, "Punpckhwd");
  cpu.unimplemented(0x31035ad, "Movq");
  cpu.unimplemented(0x31035b0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = 0xfu32; cpu.edx = v; }
  pc = 0x31035bf; continue;
 }
 0x31035bf => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x31035c7, "Movq");
  cpu.unimplemented(0x31035ce, "Movq");
  pc = 0x31035d1; continue;
 }
 0x31035d1 => {
  cpu.unimplemented(0x31035d1, "Movq");
  cpu.unimplemented(0x31035d5, "Movq");
  cpu.unimplemented(0x31035d8, "Movq");
  cpu.unimplemented(0x31035db, "Movq");
  cpu.unimplemented(0x31035de, "Psrlq");
  cpu.unimplemented(0x31035e2, "Psllq");
  cpu.unimplemented(0x31035e6, "Pand");
  cpu.unimplemented(0x31035ed, "Por");
  cpu.unimplemented(0x31035f0, "Psrlq");
  cpu.unimplemented(0x31035f4, "Pand");
  cpu.unimplemented(0x31035fb, "Psrlq");
  cpu.unimplemented(0x31035ff, "Paddw");
  cpu.unimplemented(0x3103602, "Movq");
  cpu.unimplemented(0x3103605, "Punpcklwd");
  cpu.unimplemented(0x3103608, "Punpckhwd");
  cpu.unimplemented(0x310360b, "Movq");
  cpu.unimplemented(0x310360e, "Pand");
  cpu.unimplemented(0x3103611, "Movq");
  cpu.unimplemented(0x3103618, "Movq");
  cpu.unimplemented(0x310361c, "Pand");
  cpu.unimplemented(0x310361f, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x310362c, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x31035d1; continue; }
  pc = 0x3103632; continue;
 }
 0x3103632 => {
  cpu.unimplemented(0x3103632, "Punpcklwd");
  cpu.unimplemented(0x3103635, "Punpckhwd");
  cpu.unimplemented(0x3103638, "Movq");
  cpu.unimplemented(0x310363b, "Pand");
  cpu.unimplemented(0x310363e, "Movq");
  cpu.unimplemented(0x3103645, "Movq");
  cpu.unimplemented(0x3103649, "Pand");
  cpu.unimplemented(0x310364c, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31035bf; continue; }
  pc = 0x3103666; continue;
 }
 0x3103666 => {
  pc = 0x3103717; continue;
 }
 0x310366b => {
  { let v: u32 = 0x14u32; cpu.edx = v; }
  pc = 0x3103670; continue;
 }
 0x3103670 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  cpu.unimplemented(0x3103678, "Movq");
  cpu.unimplemented(0x310367f, "Movq");
  pc = 0x3103682; continue;
 }
 0x3103682 => {
  cpu.unimplemented(0x3103682, "Movq");
  cpu.unimplemented(0x3103686, "Movq");
  cpu.unimplemented(0x3103689, "Movq");
  cpu.unimplemented(0x310368c, "Movq");
  cpu.unimplemented(0x310368f, "Psrlq");
  cpu.unimplemented(0x3103693, "Psllq");
  cpu.unimplemented(0x3103697, "Pand");
  cpu.unimplemented(0x310369e, "Por");
  cpu.unimplemented(0x31036a1, "Psrlq");
  cpu.unimplemented(0x31036a5, "Pand");
  cpu.unimplemented(0x31036ac, "Psrlq");
  cpu.unimplemented(0x31036b0, "Paddw");
  cpu.unimplemented(0x31036b3, "Movq");
  cpu.unimplemented(0x31036b6, "Punpcklwd");
  cpu.unimplemented(0x31036b9, "Punpckhwd");
  cpu.unimplemented(0x31036bc, "Movq");
  cpu.unimplemented(0x31036bf, "Pand");
  cpu.unimplemented(0x31036c2, "Movq");
  cpu.unimplemented(0x31036c9, "Movq");
  cpu.unimplemented(0x31036cd, "Pand");
  cpu.unimplemented(0x31036d0, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  cpu.unimplemented(0x31036dd, "Movq");
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x3103682; continue; }
  pc = 0x31036e3; continue;
 }
 0x31036e3 => {
  cpu.unimplemented(0x31036e3, "Punpcklwd");
  cpu.unimplemented(0x31036e6, "Punpckhwd");
  cpu.unimplemented(0x31036e9, "Movq");
  cpu.unimplemented(0x31036ec, "Pand");
  cpu.unimplemented(0x31036ef, "Movq");
  cpu.unimplemented(0x31036f6, "Movq");
  cpu.unimplemented(0x31036fa, "Pand");
  cpu.unimplemented(0x31036fd, "Movq");
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x500u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103670; continue; }
  pc = 0x3103717; continue;
 }
 0x3103717 => {
  cpu.unimplemented(0x3103717, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310371b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310371b); }
 let mut pc: u32 = 0x310371b;
 loop { match pc {
 0x310371b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103720); }
 let mut pc: u32 = 0x3103720;
 loop { match pc {
 0x3103720 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103725); }
 let mut pc: u32 = 0x3103725;
 loop { match pc {
 0x3103725 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32525dcu32); cpu.ecx = v; }
  pc = 0x3103734; continue;
 }
 0x3103734 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x3103734; continue; }
  pc = 0x3103757; continue;
 }
 0x3103757 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103759); }
 let mut pc: u32 = 0x3103759;
 loop { match pc {
 0x3103759 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x310379c; continue;
 }
 0x310379c => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(2, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310379c; continue; }
  pc = 0x31037ae; continue;
 }
 0x31037ae => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_031037b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x31037b1); }
 let mut pc: u32 = 0x31037b1;
 loop { match pc {
 0x31037b1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x31037f4; continue;
 }
 0x31037f4 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31037f4; continue; }
  pc = 0x3103805; continue;
 }
 0x3103805 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103808(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103808); }
 let mut pc: u32 = 0x3103808;
 loop { match pc {
 0x3103808 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 1u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3103823; continue; }
  pc = 0x310381d; continue;
 }
 0x310381d => {
  { let v: u32 = cpu.edi.wrapping_add(0xc800u32); cpu.edi = v; }
  pc = 0x3103823; continue;
 }
 0x3103823 => {
  { let v: u32 = cpu.r32(0x32525f8u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x310382b; continue;
 }
 0x310382b => {
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  pc = 0x3103852; continue;
 }
 0x3103836 => {
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let a = (cpu.ebx as u16); let b = (cpu.eax as u16); { let v: u16 = b; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; } { let v: u16 = a; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x500u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x504u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x3103852; continue;
 }
 0x3103852 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103836; continue; }
  pc = 0x3103855; continue;
 }
 0x3103855 => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310382b; continue; }
  pc = 0x310385a; continue;
 }
 0x310385a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310385c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310385c); }
 let mut pc: u32 = 0x310385c;
 loop { match pc {
 0x310385c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x310387b; continue; }
  pc = 0x3103878; continue;
 }
 0x3103878 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x310387b; continue;
 }
 0x310387b => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x31038a9; continue;
 }
 0x31038a9 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x31038bd; continue;
 }
 0x31038ae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.lods(2, false); }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x4fcu32.wrapping_add(cpu.edi), v); }
  pc = 0x31038bd; continue;
 }
 0x31038bd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31038ae; continue; }
  pc = 0x31038c0; continue;
 }
 0x31038c0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31038a9; continue; }
  pc = 0x31038c9; continue;
 }
 0x31038c9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_031038cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x31038cc); }
 let mut pc: u32 = 0x31038cc;
 loop { match pc {
 0x31038cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff4u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x31038eb; continue; }
  pc = 0x31038e8; continue;
 }
 0x31038e8 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x31038eb; continue;
 }
 0x31038eb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x3103919; continue;
 }
 0x3103919 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x3103921; continue;
 }
 0x310391e => {
  { cpu.lods(4, false); }
  { cpu.stos(2, false); }
  pc = 0x3103921; continue;
 }
 0x3103921 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310391e; continue; }
  pc = 0x3103924; continue;
 }
 0x3103924 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103919; continue; }
  pc = 0x310392d; continue;
 }
 0x310392d => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103930); }
 let mut pc: u32 = 0x3103930;
 loop { match pc {
 0x3103930 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x500u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e94u32, v); }
  { let v: u32 = 0xc8u32; cpu.w32(0x32f1e7cu32, v); }
  pc = 0x310394d; continue;
 }
 0x310394d => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103960; continue; }
  pc = 0x310395a; continue;
 }
 0x310395a => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103960; continue;
 }
 0x3103960 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x31039a4; continue;
 }
 0x3103973 => {
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
  pc = 0x31039a4; continue;
 }
 0x31039a4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103973; continue; }
  pc = 0x31039ac; continue;
 }
 0x31039ac => {
  { let v: u32 = cpu.dec32(cpu.r32(0x32f1e7cu32)); cpu.w32(0x32f1e7cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310394d; continue; }
  pc = 0x31039b4; continue;
 }
 0x31039b4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_031039b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x31039b6); }
 let mut pc: u32 = 0x31039b6;
 loop { match pc {
 0x31039b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x500u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e94u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x140u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e84u32, v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e88u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e8cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32f1e90u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e7cu32, v); }
  pc = 0x3103a26; continue;
 }
 0x3103a26 => {
  { let v: u32 = cpu.r32(0x32f1e8cu32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103a3a; continue; }
  pc = 0x3103a34; continue;
 }
 0x3103a34 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103a3a; continue;
 }
 0x3103a3a => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3103a7e; continue;
 }
 0x3103a4d => {
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
  pc = 0x3103a7e; continue;
 }
 0x3103a7e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103a4d; continue; }
  pc = 0x3103a86; continue;
 }
 0x3103a86 => {
  { let v: u32 = cpu.r32(0x32f1e90u32); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103a9a; continue; }
  pc = 0x3103a94; continue;
 }
 0x3103a94 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103a9a; continue;
 }
 0x3103a9a => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebx = v; }
  pc = 0x3103ab9; continue;
 }
 0x3103aa2 => {
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
  pc = 0x3103ab9; continue;
 }
 0x3103ab9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103aa2; continue; }
  pc = 0x3103abc; continue;
 }
 0x3103abc => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x32f1e84u32)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x32f1e88u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x32f1e88u32)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32f1e7cu32)); cpu.w32(0x32f1e7cu32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103a26; continue; }
  pc = 0x3103ada; continue;
 }
 0x3103ada => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103ade(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103ade); }
 let mut pc: u32 = 0x3103ade;
 loop { match pc {
 0x3103ade => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff8u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x3103b15; continue;
 }
 0x3103b15 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3103b2c; continue;
 }
 0x3103b22 => {
  { cpu.lods(1, false); }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x3103b2c; continue;
 }
 0x3103b2c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103b22; continue; }
  pc = 0x3103b2f; continue;
 }
 0x3103b2f => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103b15; continue; }
  pc = 0x3103b3a; continue;
 }
 0x3103b3a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03103b3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3103b3d); }
 let mut pc: u32 = 0x3103b3d;
 loop { match pc {
 0x3103b3d => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3103b8f; continue; }
  pc = 0x3103b51; continue;
 }
 0x3103b51 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3103b86; continue; }
  pc = 0x3103b75; continue;
 }
 0x3103b75 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3103b86; continue;
 }
 0x3103b86 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3103b8f; continue;
 }
 0x3103b8f => {
  { let v: u32 = cpu.edi; cpu.w32(0x32f1e80u32, v); }
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e94u32, v); }
  { let v: u32 = 0x33u32; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3103bc9; continue; }
  pc = 0x3103bb8; continue;
 }
 0x3103bb8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x6400u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6400u32); cpu.edx = v; }
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  pc = 0x3103bc9; continue;
 }
 0x3103bc9 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103bdd; continue; }
  pc = 0x3103bd7; continue;
 }
 0x3103bd7 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103bdd; continue;
 }
 0x3103bdd => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3103c7c; continue;
 }
 0x3103bf3 => {
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
  pc = 0x3103c7c; continue;
 }
 0x3103c7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103bf3; continue; }
  pc = 0x3103c88; continue;
 }
 0x3103c88 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103bc9; continue; }
  pc = 0x3103c9c; continue;
 }
 0x3103c9c => {
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103cb0; continue; }
  pc = 0x3103caa; continue;
 }
 0x3103caa => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103cb0; continue;
 }
 0x3103cb0 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3103d15; continue;
 }
 0x3103cc8 => {
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
  pc = 0x3103d15; continue;
 }
 0x3103d15 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103cc8; continue; }
  pc = 0x3103d1d; continue;
 }
 0x3103d1d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3103d74; continue; }
  pc = 0x3103d36; continue;
 }
 0x3103d36 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3103d6b; continue; }
  pc = 0x3103d5a; continue;
 }
 0x3103d5a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3103d6b; continue;
 }
 0x3103d6b => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3103d74; continue;
 }
 0x3103d74 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103d8f; continue; }
  pc = 0x3103d89; continue;
 }
 0x3103d89 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103d8f; continue;
 }
 0x3103d8f => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3103df4; continue;
 }
 0x3103da7 => {
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
  pc = 0x3103df4; continue;
 }
 0x3103df4 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103da7; continue; }
  pc = 0x3103dfc; continue;
 }
 0x3103dfc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103e17; continue; }
  pc = 0x3103e11; continue;
 }
 0x3103e11 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103e17; continue;
 }
 0x3103e17 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3103e7c; continue;
 }
 0x3103e2f => {
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
  pc = 0x3103e7c; continue;
 }
 0x3103e7c => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103e2f; continue; }
  pc = 0x3103e84; continue;
 }
 0x3103e84 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x3103e8a; continue;
 }
 0x3103e8a => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103e9e; continue; }
  pc = 0x3103e98; continue;
 }
 0x3103e98 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103e9e; continue;
 }
 0x3103e9e => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3103f3d; continue;
 }
 0x3103eb4 => {
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
  pc = 0x3103f3d; continue;
 }
 0x3103f3d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103eb4; continue; }
  pc = 0x3103f49; continue;
 }
 0x3103f49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3103e8a; continue; }
  pc = 0x3103f5d; continue;
 }
 0x3103f5d => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3103f71; continue; }
  pc = 0x3103f6b; continue;
 }
 0x3103f6b => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3103f71; continue;
 }
 0x3103f71 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3103fd6; continue;
 }
 0x3103f89 => {
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
  pc = 0x3103fd6; continue;
 }
 0x3103fd6 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3103f89; continue; }
  pc = 0x3103fde; continue;
 }
 0x3103fde => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3104035; continue; }
  pc = 0x3103ff7; continue;
 }
 0x3103ff7 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310402c; continue; }
  pc = 0x310401b; continue;
 }
 0x310401b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310402c; continue;
 }
 0x310402c => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3104035; continue;
 }
 0x3104035 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104050; continue; }
  pc = 0x310404a; continue;
 }
 0x310404a => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104050; continue;
 }
 0x3104050 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x31040b5; continue;
 }
 0x3104068 => {
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
  pc = 0x31040b5; continue;
 }
 0x31040b5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104068; continue; }
  pc = 0x31040bd; continue;
 }
 0x31040bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31040d8; continue; }
  pc = 0x31040d2; continue;
 }
 0x31040d2 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31040d8; continue;
 }
 0x31040d8 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x310413d; continue;
 }
 0x31040f0 => {
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
  pc = 0x310413d; continue;
 }
 0x310413d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31040f0; continue; }
  pc = 0x3104145; continue;
 }
 0x3104145 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x310414b; continue;
 }
 0x310414b => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x310415f; continue; }
  pc = 0x3104159; continue;
 }
 0x3104159 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x310415f; continue;
 }
 0x310415f => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x31041fe; continue;
 }
 0x3104175 => {
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
  pc = 0x31041fe; continue;
 }
 0x31041fe => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104175; continue; }
  pc = 0x310420a; continue;
 }
 0x310420a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310414b; continue; }
  pc = 0x310421e; continue;
 }
 0x310421e => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104232; continue; }
  pc = 0x310422c; continue;
 }
 0x310422c => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104232; continue;
 }
 0x3104232 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3104297; continue;
 }
 0x310424a => {
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
  pc = 0x3104297; continue;
 }
 0x3104297 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x310424a; continue; }
  pc = 0x310429f; continue;
 }
 0x310429f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31042ba; continue; }
  pc = 0x31042b4; continue;
 }
 0x31042b4 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31042ba; continue;
 }
 0x31042ba => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x310431f; continue;
 }
 0x31042d2 => {
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
  pc = 0x310431f; continue;
 }
 0x310431f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31042d2; continue; }
  pc = 0x3104327; continue;
 }
 0x3104327 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310437e; continue; }
  pc = 0x3104340; continue;
 }
 0x3104340 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104375; continue; }
  pc = 0x3104364; continue;
 }
 0x3104364 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104375; continue;
 }
 0x3104375 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310437e; continue;
 }
 0x310437e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x31047b5; continue; }
  pc = 0x310438f; continue;
 }
 0x310438f => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31043a9; continue; }
  pc = 0x31043a3; continue;
 }
 0x31043a3 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31043a9; continue;
 }
 0x31043a9 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x310440e; continue;
 }
 0x31043c1 => {
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
  pc = 0x310440e; continue;
 }
 0x310440e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31043c1; continue; }
  pc = 0x3104416; continue;
 }
 0x3104416 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x32u32; cpu.ebp = v; }
  pc = 0x310441c; continue;
 }
 0x310441c => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104430; continue; }
  pc = 0x310442a; continue;
 }
 0x310442a => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104430; continue;
 }
 0x3104430 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x31044cf; continue;
 }
 0x3104446 => {
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
  pc = 0x31044cf; continue;
 }
 0x31044cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104446; continue; }
  pc = 0x31044db; continue;
 }
 0x31044db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310441c; continue; }
  pc = 0x31044ef; continue;
 }
 0x31044ef => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104503; continue; }
  pc = 0x31044fd; continue;
 }
 0x31044fd => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104503; continue;
 }
 0x3104503 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3104568; continue;
 }
 0x310451b => {
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
  pc = 0x3104568; continue;
 }
 0x3104568 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x310451b; continue; }
  pc = 0x3104570; continue;
 }
 0x3104570 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x280u32); cpu.esi = v; }
  { let v: u32 = 0x60u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x310458b; continue; }
  pc = 0x3104585; continue;
 }
 0x3104585 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x310458b; continue;
 }
 0x310458b => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x31045f0; continue;
 }
 0x31045a3 => {
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
  pc = 0x31045f0; continue;
 }
 0x31045f0 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31045a3; continue; }
  pc = 0x31045f8; continue;
 }
 0x31045f8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310464f; continue; }
  pc = 0x3104611; continue;
 }
 0x3104611 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104646; continue; }
  pc = 0x3104635; continue;
 }
 0x3104635 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104646; continue;
 }
 0x3104646 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310464f; continue;
 }
 0x310464f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x310466a; continue; }
  pc = 0x3104664; continue;
 }
 0x3104664 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x310466a; continue;
 }
 0x310466a => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x31046cf; continue;
 }
 0x3104682 => {
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
  pc = 0x31046cf; continue;
 }
 0x31046cf => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104682; continue; }
  pc = 0x31046d7; continue;
 }
 0x31046d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xfu32; cpu.ebp = v; }
  pc = 0x31046dd; continue;
 }
 0x31046dd => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31046f1; continue; }
  pc = 0x31046eb; continue;
 }
 0x31046eb => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31046f1; continue;
 }
 0x31046f1 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104790; continue;
 }
 0x3104707 => {
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
  pc = 0x3104790; continue;
 }
 0x3104790 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104707; continue; }
  pc = 0x310479c; continue;
 }
 0x310479c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31046dd; continue; }
  pc = 0x31047b0; continue;
 }
 0x31047b0 => {
  pc = 0x3104915; continue;
 }
 0x31047b5 => {
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31047cf; continue; }
  pc = 0x31047c9; continue;
 }
 0x31047c9 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31047cf; continue;
 }
 0x31047cf => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = 0x3fff8u32; cpu.ecx = v; }
  pc = 0x3104834; continue;
 }
 0x31047e7 => {
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
  pc = 0x3104834; continue;
 }
 0x3104834 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31047e7; continue; }
  pc = 0x310483c; continue;
 }
 0x310483c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x2eu32; cpu.ebp = v; }
  pc = 0x3104842; continue;
 }
 0x3104842 => {
  { let v: u32 = 0xa0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104856; continue; }
  pc = 0x3104850; continue;
 }
 0x3104850 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104856; continue;
 }
 0x3104856 => {
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x31048f5; continue;
 }
 0x310486c => {
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
  pc = 0x31048f5; continue;
 }
 0x31048f5 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x310486c; continue; }
  pc = 0x3104901; continue;
 }
 0x3104901 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x280u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x280u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3104842; continue; }
  pc = 0x3104915; continue;
 }
 0x3104915 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03104918(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3104918); }
 let mut pc: u32 = 0x3104918;
 loop { match pc {
 0x3104918 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x3251b90u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e94u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3104947; continue; }
  pc = 0x3104944; continue;
 }
 0x3104944 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x3104947; continue;
 }
 0x3104947 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31049b6; continue; }
  pc = 0x3104978; continue;
 }
 0x3104978 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31049ad; continue; }
  pc = 0x310499c; continue;
 }
 0x310499c => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31049ad; continue;
 }
 0x31049ad => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31049b6; continue;
 }
 0x31049b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  pc = 0x31049ee; continue;
 }
 0x31049ee => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x3104a85; continue; }
  pc = 0x31049fa; continue;
 }
 0x31049fa => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104a0d; continue; }
  pc = 0x3104a07; continue;
 }
 0x3104a07 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104a0d; continue;
 }
 0x3104a0d => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104a64; continue;
 }
 0x3104a1f => {
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
  pc = 0x3104a64; continue;
 }
 0x3104a64 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104a1f; continue; }
  pc = 0x3104a6c; continue;
 }
 0x3104a6c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31049ee; continue; }
  pc = 0x3104a80; continue;
 }
 0x3104a80 => {
  pc = 0x3105004; continue;
 }
 0x3104a85 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x3104c2b; continue; }
  pc = 0x3104a96; continue;
 }
 0x3104a96 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104aa8; continue; }
  pc = 0x3104aa2; continue;
 }
 0x3104aa2 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104aa8; continue;
 }
 0x3104aa8 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104ae2; continue;
 }
 0x3104aba => {
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
  pc = 0x3104ae2; continue;
 }
 0x3104ae2 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104aba; continue; }
  pc = 0x3104aea; continue;
 }
 0x3104aea => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3104b43; continue; }
  pc = 0x3104b05; continue;
 }
 0x3104b05 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104b3a; continue; }
  pc = 0x3104b29; continue;
 }
 0x3104b29 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104b3a; continue;
 }
 0x3104b3a => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3104b43; continue;
 }
 0x3104b43 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104b67; continue; }
  pc = 0x3104b61; continue;
 }
 0x3104b61 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104b67; continue;
 }
 0x3104b67 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104ba1; continue;
 }
 0x3104b79 => {
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
  pc = 0x3104ba1; continue;
 }
 0x3104ba1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104b79; continue; }
  pc = 0x3104ba9; continue;
 }
 0x3104ba9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104bca; continue; }
  pc = 0x3104bc4; continue;
 }
 0x3104bc4 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104bca; continue;
 }
 0x3104bca => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104c04; continue;
 }
 0x3104bdc => {
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
  pc = 0x3104c04; continue;
 }
 0x3104c04 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104bdc; continue; }
  pc = 0x3104c0c; continue;
 }
 0x3104c0c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31049ee; continue; }
  pc = 0x3104c26; continue;
 }
 0x3104c26 => {
  pc = 0x3105004; continue;
 }
 0x3104c2b => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x3104ed1; continue; }
  pc = 0x3104c39; continue;
 }
 0x3104c39 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x3104ddf; continue; }
  pc = 0x3104c42; continue;
 }
 0x3104c42 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104c55; continue; }
  pc = 0x3104c4f; continue;
 }
 0x3104c4f => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104c55; continue;
 }
 0x3104c55 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104c8f; continue;
 }
 0x3104c67 => {
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
  pc = 0x3104c8f; continue;
 }
 0x3104c8f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104c67; continue; }
  pc = 0x3104c97; continue;
 }
 0x3104c97 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104cbf; continue; }
  pc = 0x3104cb9; continue;
 }
 0x3104cb9 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104cbf; continue;
 }
 0x3104cbf => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104cf9; continue;
 }
 0x3104cd1 => {
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
  pc = 0x3104cf9; continue;
 }
 0x3104cf9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104cd1; continue; }
  pc = 0x3104d01; continue;
 }
 0x3104d01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3104d5a; continue; }
  pc = 0x3104d1c; continue;
 }
 0x3104d1c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104d51; continue; }
  pc = 0x3104d40; continue;
 }
 0x3104d40 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104d51; continue;
 }
 0x3104d51 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3104d5a; continue;
 }
 0x3104d5a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104d7e; continue; }
  pc = 0x3104d78; continue;
 }
 0x3104d78 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104d7e; continue;
 }
 0x3104d7e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104db8; continue;
 }
 0x3104d90 => {
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
  pc = 0x3104db8; continue;
 }
 0x3104db8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104d90; continue; }
  pc = 0x3104dc0; continue;
 }
 0x3104dc0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31049ee; continue; }
  pc = 0x3104dda; continue;
 }
 0x3104dda => {
  pc = 0x3105004; continue;
 }
 0x3104ddf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104df2; continue; }
  pc = 0x3104dec; continue;
 }
 0x3104dec => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104df2; continue;
 }
 0x3104df2 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104e49; continue;
 }
 0x3104e04 => {
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
  pc = 0x3104e49; continue;
 }
 0x3104e49 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104e04; continue; }
  pc = 0x3104e51; continue;
 }
 0x3104e51 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3104eaa; continue; }
  pc = 0x3104e6c; continue;
 }
 0x3104e6c => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104ea1; continue; }
  pc = 0x3104e90; continue;
 }
 0x3104e90 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104ea1; continue;
 }
 0x3104ea1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3104eaa; continue;
 }
 0x3104eaa => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31049ee; continue; }
  pc = 0x3104ecc; continue;
 }
 0x3104ecc => {
  pc = 0x3105004; continue;
 }
 0x3104ed1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104ee4; continue; }
  pc = 0x3104ede; continue;
 }
 0x3104ede => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104ee4; continue;
 }
 0x3104ee4 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104f1e; continue;
 }
 0x3104ef6 => {
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
  pc = 0x3104f1e; continue;
 }
 0x3104f1e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104ef6; continue; }
  pc = 0x3104f26; continue;
 }
 0x3104f26 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3104f85; continue; }
  pc = 0x3104f47; continue;
 }
 0x3104f47 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3104f7c; continue; }
  pc = 0x3104f6b; continue;
 }
 0x3104f6b => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3104f7c; continue;
 }
 0x3104f7c => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3104f85; continue;
 }
 0x3104f85 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3104fae; continue; }
  pc = 0x3104fa8; continue;
 }
 0x3104fa8 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3104fae; continue;
 }
 0x3104fae => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32f1e78u32, v); }
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  pc = 0x3104fe8; continue;
 }
 0x3104fc0 => {
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
  pc = 0x3104fe8; continue;
 }
 0x3104fe8 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3104fc0; continue; }
  pc = 0x3104ff0; continue;
 }
 0x3104ff0 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31049ee; continue; }
  pc = 0x3105004; continue;
 }
 0x3105004 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105007); }
 let mut pc: u32 = 0x3105007;
 loop { match pc {
 0x3105007 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xfffffff0u32); cpu.esp = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32525b0u32), 0x1e0u32); }
  if !cpu.zf { pc = 0x3105026; continue; }
  pc = 0x3105023; continue;
 }
 0x3105023 => {
  { let v: u32 = cpu.eax.wrapping_add(0x14u32); cpu.eax = v; }
  pc = 0x3105026; continue;
 }
 0x3105026 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x32525fcu32); cpu.mul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x32525d8u32); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v = cpu.r32(0x32525f0u32); cpu.mul32(v); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310508e; continue; }
  pc = 0x3105050; continue;
 }
 0x3105050 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105085; continue; }
  pc = 0x3105074; continue;
 }
 0x3105074 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105085; continue;
 }
 0x3105085 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310508e; continue;
 }
 0x310508e => {
  { let v: u32 = cpu.r32(0x32525d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32525b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x31050c8; continue;
 }
 0x31050c8 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x32525f4u32)); cpu.ebx = v; }
  if cpu.sf { pc = 0x31050fb; continue; }
  pc = 0x31050d0; continue;
 }
 0x31050d0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x31050e9; continue;
 }
 0x31050de => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x31050e9; continue;
 }
 0x31050e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050de; continue; }
  pc = 0x31050ec; continue;
 }
 0x31050ec => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050c8; continue; }
  pc = 0x31050f6; continue;
 }
 0x31050f6 => {
  pc = 0x3105427; continue;
 }
 0x31050fb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525f4u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x31051e8; continue; }
  pc = 0x310510c; continue;
 }
 0x310510c => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3105124; continue;
 }
 0x3105119 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x3105124; continue;
 }
 0x3105124 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105119; continue; }
  pc = 0x3105127; continue;
 }
 0x3105127 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310517f; continue; }
  pc = 0x3105141; continue;
 }
 0x3105141 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105176; continue; }
  pc = 0x3105165; continue;
 }
 0x3105165 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105176; continue;
 }
 0x3105176 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310517f; continue;
 }
 0x310517f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525f4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x31051a9; continue;
 }
 0x310519e => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x31051a9; continue;
 }
 0x31051a9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310519e; continue; }
  pc = 0x31051ac; continue;
 }
 0x31051ac => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x31051cc; continue;
 }
 0x31051c1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x31051cc; continue;
 }
 0x31051cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31051c1; continue; }
  pc = 0x31051cf; continue;
 }
 0x31051cf => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050c8; continue; }
  pc = 0x31051e3; continue;
 }
 0x31051e3 => {
  pc = 0x3105427; continue;
 }
 0x31051e8 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  if cpu.sf { pc = 0x3105373; continue; }
  pc = 0x31051f6; continue;
 }
 0x31051f6 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf == cpu.of) { pc = 0x31052e3; continue; }
  pc = 0x31051ff; continue;
 }
 0x31051ff => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3105218; continue;
 }
 0x310520d => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x3105218; continue;
 }
 0x3105218 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310520d; continue; }
  pc = 0x310521b; continue;
 }
 0x310521b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32525b4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3105242; continue;
 }
 0x3105237 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x3105242; continue;
 }
 0x3105242 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105237; continue; }
  pc = 0x3105245; continue;
 }
 0x3105245 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310529d; continue; }
  pc = 0x310525f; continue;
 }
 0x310525f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105294; continue; }
  pc = 0x3105283; continue;
 }
 0x3105283 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105294; continue;
 }
 0x3105294 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310529d; continue;
 }
 0x310529d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32525b4u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x31052c7; continue;
 }
 0x31052bc => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x31052c7; continue;
 }
 0x31052c7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31052bc; continue; }
  pc = 0x31052ca; continue;
 }
 0x31052ca => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050c8; continue; }
  pc = 0x31052de; continue;
 }
 0x31052de => {
  pc = 0x3105427; continue;
 }
 0x31052e3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x31052fc; continue;
 }
 0x31052f1 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x31052fc; continue;
 }
 0x31052fc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x31052f1; continue; }
  pc = 0x31052ff; continue;
 }
 0x31052ff => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3105357; continue; }
  pc = 0x3105319; continue;
 }
 0x3105319 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x310534e; continue; }
  pc = 0x310533d; continue;
 }
 0x310533d => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x310534e; continue;
 }
 0x310534e => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3105357; continue;
 }
 0x3105357 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050c8; continue; }
  pc = 0x310536e; continue;
 }
 0x310536e => {
  pc = 0x3105427; continue;
 }
 0x3105373 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x310538c; continue;
 }
 0x3105381 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x310538c; continue;
 }
 0x310538c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105381; continue; }
  pc = 0x310538f; continue;
 }
 0x310538f => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31053ea; continue; }
  pc = 0x31053ac; continue;
 }
 0x31053ac => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31053e1; continue; }
  pc = 0x31053d0; continue;
 }
 0x31053d0 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31053e1; continue;
 }
 0x31053e1 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31053ea; continue;
 }
 0x31053ea => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x32525b4u32)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x32525d8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3252610u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3105416; continue;
 }
 0x310540b => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  pc = 0x3105416; continue;
 }
 0x3105416 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x310540b; continue; }
  pc = 0x3105419; continue;
 }
 0x3105419 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31050c8; continue; }
  pc = 0x3105427; continue;
 }
 0x3105427 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0310542a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x310542a); }
 let mut pc: u32 = 0x310542a;
 loop { match pc {
 0x310542a => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.edx as u16) ^ (cpu.edx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310547c; continue; }
  pc = 0x310543e; continue;
 }
 0x310543e => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105473; continue; }
  pc = 0x3105462; continue;
 }
 0x3105462 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105473; continue;
 }
 0x3105473 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310547c; continue;
 }
 0x310547c => {
  { let v: u32 = cpu.edi; cpu.w32(0x32f1e80u32, v); }
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xa00u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e94u32, v); }
  { let v: u32 = 0x66u32; cpu.ebp = v; }
  pc = 0x3105499; continue;
 }
 0x3105499 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31054ad; continue; }
  pc = 0x31054a7; continue;
 }
 0x31054a7 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31054ad; continue;
 }
 0x31054ad => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x31054f1; continue;
 }
 0x31054c0 => {
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
  pc = 0x31054f1; continue;
 }
 0x31054f1 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31054c0; continue; }
  pc = 0x31054f9; continue;
 }
 0x31054f9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105499; continue; }
  pc = 0x31054fd; continue;
 }
 0x31054fd => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105511; continue; }
  pc = 0x310550b; continue;
 }
 0x310550b => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105511; continue;
 }
 0x3105511 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105555; continue;
 }
 0x3105524 => {
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
  pc = 0x3105555; continue;
 }
 0x3105555 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105524; continue; }
  pc = 0x310555d; continue;
 }
 0x310555d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31055b4; continue; }
  pc = 0x3105576; continue;
 }
 0x3105576 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31055ab; continue; }
  pc = 0x310559a; continue;
 }
 0x310559a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31055ab; continue;
 }
 0x31055ab => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31055b4; continue;
 }
 0x31055b4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31055cf; continue; }
  pc = 0x31055c9; continue;
 }
 0x31055c9 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31055cf; continue;
 }
 0x31055cf => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105613; continue;
 }
 0x31055e2 => {
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
  pc = 0x3105613; continue;
 }
 0x3105613 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31055e2; continue; }
  pc = 0x310561b; continue;
 }
 0x310561b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105630; continue; }
  pc = 0x310562a; continue;
 }
 0x310562a => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105630; continue;
 }
 0x3105630 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105674; continue;
 }
 0x3105643 => {
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
  pc = 0x3105674; continue;
 }
 0x3105674 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105643; continue; }
  pc = 0x310567c; continue;
 }
 0x310567c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x3105682; continue;
 }
 0x3105682 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105696; continue; }
  pc = 0x3105690; continue;
 }
 0x3105690 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105696; continue;
 }
 0x3105696 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x31056da; continue;
 }
 0x31056a9 => {
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
  pc = 0x31056da; continue;
 }
 0x31056da => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31056a9; continue; }
  pc = 0x31056e2; continue;
 }
 0x31056e2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105682; continue; }
  pc = 0x31056e6; continue;
 }
 0x31056e6 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31056fa; continue; }
  pc = 0x31056f4; continue;
 }
 0x31056f4 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31056fa; continue;
 }
 0x31056fa => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x310573e; continue;
 }
 0x310570d => {
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
  pc = 0x310573e; continue;
 }
 0x310573e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x310570d; continue; }
  pc = 0x3105746; continue;
 }
 0x3105746 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x310579d; continue; }
  pc = 0x310575f; continue;
 }
 0x310575f => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105794; continue; }
  pc = 0x3105783; continue;
 }
 0x3105783 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105794; continue;
 }
 0x3105794 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x310579d; continue;
 }
 0x310579d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31057b8; continue; }
  pc = 0x31057b2; continue;
 }
 0x31057b2 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31057b8; continue;
 }
 0x31057b8 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x31057fc; continue;
 }
 0x31057cb => {
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
  pc = 0x31057fc; continue;
 }
 0x31057fc => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31057cb; continue; }
  pc = 0x3105804; continue;
 }
 0x3105804 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105819; continue; }
  pc = 0x3105813; continue;
 }
 0x3105813 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105819; continue;
 }
 0x3105819 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x310585d; continue;
 }
 0x310582c => {
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
  pc = 0x310585d; continue;
 }
 0x310585d => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x310582c; continue; }
  pc = 0x3105865; continue;
 }
 0x3105865 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x310586b; continue;
 }
 0x310586b => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x310587f; continue; }
  pc = 0x3105879; continue;
 }
 0x3105879 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x310587f; continue;
 }
 0x310587f => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x31058c3; continue;
 }
 0x3105892 => {
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
  pc = 0x31058c3; continue;
 }
 0x31058c3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105892; continue; }
  pc = 0x31058cb; continue;
 }
 0x31058cb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x310586b; continue; }
  pc = 0x31058cf; continue;
 }
 0x31058cf => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x31058e3; continue; }
  pc = 0x31058dd; continue;
 }
 0x31058dd => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x31058e3; continue;
 }
 0x31058e3 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105927; continue;
 }
 0x31058f6 => {
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
  pc = 0x3105927; continue;
 }
 0x3105927 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x31058f6; continue; }
  pc = 0x310592f; continue;
 }
 0x310592f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105944; continue; }
  pc = 0x310593e; continue;
 }
 0x310593e => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105944; continue;
 }
 0x3105944 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105988; continue;
 }
 0x3105957 => {
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
  pc = 0x3105988; continue;
 }
 0x3105988 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105957; continue; }
  pc = 0x3105990; continue;
 }
 0x3105990 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x31059e7; continue; }
  pc = 0x31059a9; continue;
 }
 0x31059a9 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x31059de; continue; }
  pc = 0x31059cd; continue;
 }
 0x31059cd => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x31059de; continue;
 }
 0x31059de => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x31059e7; continue;
 }
 0x31059e7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105a02; continue; }
  pc = 0x31059fc; continue;
 }
 0x31059fc => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105a02; continue;
 }
 0x3105a02 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105a46; continue;
 }
 0x3105a15 => {
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
  pc = 0x3105a46; continue;
 }
 0x3105a46 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105a15; continue; }
  pc = 0x3105a4e; continue;
 }
 0x3105a4e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x64u32; cpu.ebp = v; }
  pc = 0x3105a54; continue;
 }
 0x3105a54 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105a68; continue; }
  pc = 0x3105a62; continue;
 }
 0x3105a62 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105a68; continue;
 }
 0x3105a68 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105aac; continue;
 }
 0x3105a7b => {
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
  pc = 0x3105aac; continue;
 }
 0x3105aac => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105a7b; continue; }
  pc = 0x3105ab4; continue;
 }
 0x3105ab4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105a54; continue; }
  pc = 0x3105ab8; continue;
 }
 0x3105ab8 => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105acc; continue; }
  pc = 0x3105ac6; continue;
 }
 0x3105ac6 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105acc; continue;
 }
 0x3105acc => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105b10; continue;
 }
 0x3105adf => {
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
  pc = 0x3105b10; continue;
 }
 0x3105b10 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105adf; continue; }
  pc = 0x3105b18; continue;
 }
 0x3105b18 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0xc0u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105b2d; continue; }
  pc = 0x3105b27; continue;
 }
 0x3105b27 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105b2d; continue;
 }
 0x3105b2d => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105b71; continue;
 }
 0x3105b40 => {
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
  pc = 0x3105b71; continue;
 }
 0x3105b71 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105b40; continue; }
  pc = 0x3105b79; continue;
 }
 0x3105b79 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32525f0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x325264au32)); }
  if cpu.zf { pc = 0x3105bd0; continue; }
  pc = 0x3105b92; continue;
 }
 0x3105b92 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x3251b74u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105bc7; continue; }
  pc = 0x3105bb6; continue;
 }
 0x3105bb6 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105bc7; continue;
 }
 0x3105bc7 => {
  { let v: u16 = 0x0u16; cpu.w16(0x3251b74u32, v); }
  pc = 0x3105bd0; continue;
 }
 0x3105bd0 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32f1e80u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105beb; continue; }
  pc = 0x3105be5; continue;
 }
 0x3105be5 => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105beb; continue;
 }
 0x3105beb => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105c2f; continue;
 }
 0x3105bfe => {
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
  pc = 0x3105c2f; continue;
 }
 0x3105c2f => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105bfe; continue; }
  pc = 0x3105c37; continue;
 }
 0x3105c37 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = 0x46u32; cpu.ebp = v; }
  pc = 0x3105c3d; continue;
 }
 0x3105c3d => {
  { let v: u32 = 0x140u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32f1e94u32)); }
  if cpu.cf { pc = 0x3105c51; continue; }
  pc = 0x3105c4b; continue;
 }
 0x3105c4b => {
  { let v: u32 = 0x3251b90u32; cpu.edx = v; }
  pc = 0x3105c51; continue;
 }
 0x3105c51 => {
  { let v: u32 = cpu.r32(0x31543f8u32); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32f1e78u32, v); }
  pc = 0x3105c95; continue;
 }
 0x3105c64 => {
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
  pc = 0x3105c95; continue;
 }
 0x3105c95 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x32f1e78u32)); }
  if cpu.cf { cpu.poll(); pc = 0x3105c64; continue; }
  pc = 0x3105c9d; continue;
 }
 0x3105c9d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3105c3d; continue; }
  pc = 0x3105ca1; continue;
 }
 0x3105ca1 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105ca4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105ca4); }
 let mut pc: u32 = 0x3105ca4;
 loop { match pc {
 0x3105ca4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105cac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105cac); }
 let mut pc: u32 = 0x3105cac;
 loop { match pc {
 0x3105cac => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xfffffff0u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105cb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105cb4); }
 let mut pc: u32 = 0x3105cb4;
 loop { match pc {
 0x3105cb4 => {
  { let v: u16 = (cpu.eax as u16).wrapping_sub((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3c8u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x3105cbd; continue;
 }
 0x3105cbd => {
  pc = 0x3105cbf; continue;
 }
 0x3105cbf => {
  pc = 0x3105cc1; continue;
 }
 0x3105cc1 => {
  {  }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  pc = 0x3105cc3; continue;
 }
 0x3105cc3 => {
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x3105ccc; continue;
 }
 0x3105ccc => {
  pc = 0x3105cce; continue;
 }
 0x3105cce => {
  pc = 0x3105cd0; continue;
 }
 0x3105cd0 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x3105cde; continue;
 }
 0x3105cde => {
  pc = 0x3105ce0; continue;
 }
 0x3105ce0 => {
  pc = 0x3105ce2; continue;
 }
 0x3105ce2 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x3105cf0; continue;
 }
 0x3105cf0 => {
  pc = 0x3105cf2; continue;
 }
 0x3105cf2 => {
  pc = 0x3105cf4; continue;
 }
 0x3105cf4 => {
  { let v: u16 = 0x3c9u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { cpu.lods(1, false); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { cpu.poll(); pc = 0x3105cc3; continue; }
  pc = 0x3105cfc; continue;
 }
 0x3105cfc => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105cfe); }
 let mut pc: u32 = 0x3105cfe;
 loop { match pc {
 0x3105cfe => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x3105d02; continue;
 }
 0x3105d02 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x3105d02; continue; }
  pc = 0x3105d07; continue;
 }
 0x3105d07 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105d12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105d12); }
 let mut pc: u32 = 0x3105d12;
 loop { match pc {
 0x3105d12 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { pc = 0x3105d20; continue; }
  pc = 0x3105d1b; continue;
 }
 0x3105d1b => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x3105d1b; continue; }
  pc = 0x3105d20; continue;
 }
 0x3105d20 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x3105d20; continue; }
  pc = 0x3105d25; continue;
 }
 0x3105d25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105d26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105d26); }
 let mut pc: u32 = 0x3105d26;
 loop { match pc {
 0x3105d26 => {
  { let v: u16 = 0x3dau16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x3105d34; continue; }
  pc = 0x3105d2f; continue;
 }
 0x3105d2f => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if !cpu.zf { cpu.poll(); pc = 0x3105d2f; continue; }
  pc = 0x3105d34; continue;
 }
 0x3105d34 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { cpu.poll(); pc = 0x3105d34; continue; }
  pc = 0x3105d39; continue;
 }
 0x3105d39 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105d3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105d3a); }
 let mut pc: u32 = 0x3105d3a;
 loop { match pc {
 0x3105d3a => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3252650u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03105d47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3105d47); }
 let mut pc: u32 = 0x3105d47;
 loop { match pc {
 0x3105d47 => {
  { let v: u16 = cpu.r16(0x3252650u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16(cpu.r16(0x325264au32), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x3105d83; continue; }
  pc = 0x3105d57; continue;
 }
 0x3105d57 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x325264au32, v); }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16) ^ (cpu.ebx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  { cpu.and16(cpu.r16(0x325264cu32), 0xffffu16); }
  if cpu.zf { pc = 0x3105d83; continue; }
  pc = 0x3105d72; continue;
 }
 0x3105d72 => {
  { let v: u16 = cpu.r16(0x325264au32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x4f05u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x10); }
  pc = 0x3105d83; continue;
 }
 0x3105d83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03110000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3110000); }
 let mut pc: u32 = 0x3110000;
 loop { match pc {
 0x3110000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x3110003; continue;
 }
 0x3110003 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x3110011; continue; }
  pc = 0x311000e; continue;
 }
 0x311000e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x3110011; continue;
 }
 0x3110011 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3110003; continue; }
  pc = 0x3110020; continue;
 }
 0x3110020 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03110022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3110022); }
 let mut pc: u32 = 0x3110022;
 loop { match pc {
 0x3110022 => {
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
pub fn f_03110038(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3110038); }
 let mut pc: u32 = 0x3110038;
 loop { match pc {
 0x3110038 => {
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
  { let v: u32 = 0x0u32; cpu.w32(0x3251b2cu32, v); }
  pc = 0x3110064; continue;
 }
 0x3110064 => {
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.r32(0x3251b2cu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x3110080; continue; }
  pc = 0x3110071; continue;
 }
 0x3110071 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x3251b2cu32, v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3251b2cu32, v); }
  pc = 0x3110080; continue;
 }
 0x3110080 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x3110085; continue; }
  pc = 0x3110084; continue;
 }
 0x3110084 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x3110085; continue;
 }
 0x3110085 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x311008a; continue; }
  pc = 0x3110089; continue;
 }
 0x3110089 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x311008a; continue;
 }
 0x311008a => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x311008f; continue; }
  pc = 0x311008e; continue;
 }
 0x311008e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x311008f; continue;
 }
 0x311008f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x3110094; continue; }
  pc = 0x3110093; continue;
 }
 0x3110093 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x3110094; continue;
 }
 0x3110094 => {
  { cpu.and8(((cpu.eax >> 8) as u8), (cpu.eax as u8)); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { cpu.poll(); pc = 0x3110064; continue; }
  pc = 0x3110098; continue;
 }
 0x3110098 => {
  {  }
  { let v: u32 = cpu.esi; cpu.w32(0x3251b50u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3251b4cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3251b54u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x3251b30u32, v); }
  { let v: u8 = cpu.r8(0x3251b2cu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = !(cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3251b2cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03120000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3120000); }
 let mut pc: u32 = 0x3120000;
 loop { match pc {
 0x3120000 => {
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x3120031; continue; }
  pc = 0x3120013; continue;
 }
 0x3120013 => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  if cpu.cf { pc = 0x3120021; continue; }
  pc = 0x3120017; continue;
 }
 0x3120017 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3120017; continue; }
  pc = 0x3120020; continue;
 }
 0x3120020 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x3120021 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  pc = 0x3120024; continue;
 }
 0x3120024 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3120024; continue; }
  pc = 0x3120030; continue;
 }
 0x3120030 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x3120031 => {
  { let v: u32 = cpu.shr32(cpu.esi, 1u32); cpu.esi = v; }
  if cpu.cf { pc = 0x3120048; continue; }
  pc = 0x3120035; continue;
 }
 0x3120035 => {
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  pc = 0x3120039; continue;
 }
 0x3120039 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { cpu.stos(2, false); }
  { cpu.stos(4, true); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3120039; continue; }
  pc = 0x3120047; continue;
 }
 0x3120047 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x3120048 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { cpu.stos(2, false); }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3120048; continue; }
  pc = 0x3120053; continue;
 }
 0x3120053 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03120054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3120054); }
 let mut pc: u32 = 0x3120054;
 loop { match pc {
 0x3120054 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x3120c24u32); cpu.ebx = v; }
  cpu.poll(); pc = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03120066(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120066); } r_03120066(cpu, 0x3120066); }
pub fn f_0312006b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312006b); } r_03120066(cpu, 0x312006b); }
pub fn f_03120070(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120070); } r_03120066(cpu, 0x3120070); }
pub fn f_03120075(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120075); } r_03120066(cpu, 0x3120075); }
pub fn f_0312007a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312007a); } r_03120066(cpu, 0x312007a); }
pub fn f_0312007f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312007f); } r_03120066(cpu, 0x312007f); }
pub fn f_03120084(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120084); } r_03120066(cpu, 0x3120084); }
pub fn f_03120089(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120089); } r_03120066(cpu, 0x3120089); }
pub fn f_0312008e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312008e); } r_03120066(cpu, 0x312008e); }
pub fn f_03120093(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120093); } r_03120066(cpu, 0x3120093); }
pub fn f_03120098(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120098); } r_03120066(cpu, 0x3120098); }
pub fn f_0312009d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312009d); } r_03120066(cpu, 0x312009d); }
pub fn f_031200a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200a2); } r_03120066(cpu, 0x31200a2); }
pub fn f_031200a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200a7); } r_03120066(cpu, 0x31200a7); }
pub fn f_031200ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200ac); } r_03120066(cpu, 0x31200ac); }
pub fn f_031200b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200b1); } r_03120066(cpu, 0x31200b1); }
pub fn f_031200b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200b6); } r_03120066(cpu, 0x31200b6); }
pub fn f_031200bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200bb); } r_03120066(cpu, 0x31200bb); }
pub fn f_031200c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200c0); } r_03120066(cpu, 0x31200c0); }
pub fn f_031200c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200c5); } r_03120066(cpu, 0x31200c5); }
pub fn f_031200ca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200ca); } r_03120066(cpu, 0x31200ca); }
pub fn f_031200cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200cf); } r_03120066(cpu, 0x31200cf); }
pub fn f_031200d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200d4); } r_03120066(cpu, 0x31200d4); }
pub fn f_031200d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200d9); } r_03120066(cpu, 0x31200d9); }
pub fn f_031200de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200de); } r_03120066(cpu, 0x31200de); }
pub fn f_031200e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200e3); } r_03120066(cpu, 0x31200e3); }
pub fn f_031200e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200e8); } r_03120066(cpu, 0x31200e8); }
pub fn f_031200ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200ed); } r_03120066(cpu, 0x31200ed); }
pub fn f_031200f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200f2); } r_03120066(cpu, 0x31200f2); }
pub fn f_031200f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200f7); } r_03120066(cpu, 0x31200f7); }
pub fn f_031200fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31200fc); } r_03120066(cpu, 0x31200fc); }
pub fn f_03120101(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120101); } r_03120066(cpu, 0x3120101); }
pub fn f_03120106(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120106); } r_03120066(cpu, 0x3120106); }
pub fn f_0312010b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312010b); } r_03120066(cpu, 0x312010b); }
pub fn f_03120110(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120110); } r_03120066(cpu, 0x3120110); }
pub fn f_03120115(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120115); } r_03120066(cpu, 0x3120115); }
pub fn f_0312011a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312011a); } r_03120066(cpu, 0x312011a); }
pub fn f_0312011f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312011f); } r_03120066(cpu, 0x312011f); }
pub fn f_03120124(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120124); } r_03120066(cpu, 0x3120124); }
pub fn f_03120129(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120129); } r_03120066(cpu, 0x3120129); }
pub fn f_0312012e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312012e); } r_03120066(cpu, 0x312012e); }
pub fn f_03120133(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120133); } r_03120066(cpu, 0x3120133); }
pub fn f_03120138(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120138); } r_03120066(cpu, 0x3120138); }
pub fn f_0312013d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312013d); } r_03120066(cpu, 0x312013d); }
pub fn f_03120142(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120142); } r_03120066(cpu, 0x3120142); }
pub fn f_03120147(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120147); } r_03120066(cpu, 0x3120147); }
pub fn f_0312014c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312014c); } r_03120066(cpu, 0x312014c); }
pub fn f_03120151(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120151); } r_03120066(cpu, 0x3120151); }
pub fn f_03120156(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120156); } r_03120066(cpu, 0x3120156); }
pub fn f_0312015b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312015b); } r_03120066(cpu, 0x312015b); }
pub fn f_03120160(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120160); } r_03120066(cpu, 0x3120160); }
pub fn f_03120165(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120165); } r_03120066(cpu, 0x3120165); }
pub fn f_0312016a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312016a); } r_03120066(cpu, 0x312016a); }
pub fn f_0312016f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312016f); } r_03120066(cpu, 0x312016f); }
pub fn f_03120174(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120174); } r_03120066(cpu, 0x3120174); }
pub fn f_03120179(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120179); } r_03120066(cpu, 0x3120179); }
pub fn f_0312017e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312017e); } r_03120066(cpu, 0x312017e); }
pub fn f_03120183(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120183); } r_03120066(cpu, 0x3120183); }
pub fn f_03120188(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120188); } r_03120066(cpu, 0x3120188); }
pub fn f_0312018d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312018d); } r_03120066(cpu, 0x312018d); }
pub fn f_03120192(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120192); } r_03120066(cpu, 0x3120192); }
pub fn f_03120197(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120197); } r_03120066(cpu, 0x3120197); }
pub fn f_0312019c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312019c); } r_03120066(cpu, 0x312019c); }
pub fn f_031201a1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201a1); } r_03120066(cpu, 0x31201a1); }
pub fn f_031201a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201a6); } r_03120066(cpu, 0x31201a6); }
pub fn f_031201ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201ab); } r_03120066(cpu, 0x31201ab); }
pub fn f_031201b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201b0); } r_03120066(cpu, 0x31201b0); }
pub fn f_031201b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201b5); } r_03120066(cpu, 0x31201b5); }
pub fn f_031201ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201ba); } r_03120066(cpu, 0x31201ba); }
pub fn f_031201bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201bf); } r_03120066(cpu, 0x31201bf); }
pub fn f_031201c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201c4); } r_03120066(cpu, 0x31201c4); }
pub fn f_031201c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201c9); } r_03120066(cpu, 0x31201c9); }
pub fn f_031201ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201ce); } r_03120066(cpu, 0x31201ce); }
pub fn f_031201d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201d3); } r_03120066(cpu, 0x31201d3); }
pub fn f_031201d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201d8); } r_03120066(cpu, 0x31201d8); }
pub fn f_031201dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201dd); } r_03120066(cpu, 0x31201dd); }
pub fn f_031201e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201e2); } r_03120066(cpu, 0x31201e2); }
pub fn f_031201e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201e7); } r_03120066(cpu, 0x31201e7); }
pub fn f_031201ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201ec); } r_03120066(cpu, 0x31201ec); }
pub fn f_031201f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201f1); } r_03120066(cpu, 0x31201f1); }
pub fn f_031201f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201f6); } r_03120066(cpu, 0x31201f6); }
pub fn f_031201fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31201fb); } r_03120066(cpu, 0x31201fb); }
pub fn f_03120200(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120200); } r_03120066(cpu, 0x3120200); }
pub fn f_03120205(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120205); } r_03120066(cpu, 0x3120205); }
pub fn f_0312020a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312020a); } r_03120066(cpu, 0x312020a); }
pub fn f_0312020f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312020f); } r_03120066(cpu, 0x312020f); }
pub fn f_03120214(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120214); } r_03120066(cpu, 0x3120214); }
pub fn f_03120219(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120219); } r_03120066(cpu, 0x3120219); }
pub fn f_0312021e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312021e); } r_03120066(cpu, 0x312021e); }
pub fn f_03120223(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120223); } r_03120066(cpu, 0x3120223); }
pub fn f_03120228(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120228); } r_03120066(cpu, 0x3120228); }
pub fn f_0312022d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312022d); } r_03120066(cpu, 0x312022d); }
pub fn f_03120232(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120232); } r_03120066(cpu, 0x3120232); }
pub fn f_03120237(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120237); } r_03120066(cpu, 0x3120237); }
pub fn f_0312023c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312023c); } r_03120066(cpu, 0x312023c); }
pub fn f_03120241(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120241); } r_03120066(cpu, 0x3120241); }
pub fn f_03120246(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120246); } r_03120066(cpu, 0x3120246); }
pub fn f_0312024b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312024b); } r_03120066(cpu, 0x312024b); }
pub fn f_03120250(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120250); } r_03120066(cpu, 0x3120250); }
pub fn f_03120255(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120255); } r_03120066(cpu, 0x3120255); }
pub fn f_0312025a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312025a); } r_03120066(cpu, 0x312025a); }
pub fn f_0312025f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312025f); } r_03120066(cpu, 0x312025f); }
pub fn f_03120264(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120264); } r_03120066(cpu, 0x3120264); }
pub fn f_03120269(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120269); } r_03120066(cpu, 0x3120269); }
pub fn f_0312026e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312026e); } r_03120066(cpu, 0x312026e); }
pub fn f_03120273(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120273); } r_03120066(cpu, 0x3120273); }
pub fn f_03120278(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120278); } r_03120066(cpu, 0x3120278); }
pub fn f_0312027d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312027d); } r_03120066(cpu, 0x312027d); }
pub fn f_03120282(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120282); } r_03120066(cpu, 0x3120282); }
pub fn f_03120287(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120287); } r_03120066(cpu, 0x3120287); }
pub fn f_0312028c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312028c); } r_03120066(cpu, 0x312028c); }
pub fn f_03120291(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120291); } r_03120066(cpu, 0x3120291); }
pub fn f_03120296(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120296); } r_03120066(cpu, 0x3120296); }
pub fn f_0312029b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312029b); } r_03120066(cpu, 0x312029b); }
pub fn f_031202a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202a0); } r_03120066(cpu, 0x31202a0); }
pub fn f_031202a5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202a5); } r_03120066(cpu, 0x31202a5); }
pub fn f_031202aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202aa); } r_03120066(cpu, 0x31202aa); }
pub fn f_031202af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202af); } r_03120066(cpu, 0x31202af); }
pub fn f_031202b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202b4); } r_03120066(cpu, 0x31202b4); }
pub fn f_031202b9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202b9); } r_03120066(cpu, 0x31202b9); }
pub fn f_031202be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202be); } r_03120066(cpu, 0x31202be); }
pub fn f_031202c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202c3); } r_03120066(cpu, 0x31202c3); }
pub fn f_031202c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202c8); } r_03120066(cpu, 0x31202c8); }
pub fn f_031202cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202cd); } r_03120066(cpu, 0x31202cd); }
pub fn f_031202d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202d2); } r_03120066(cpu, 0x31202d2); }
pub fn f_031202d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202d7); } r_03120066(cpu, 0x31202d7); }
pub fn f_031202dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202dc); } r_03120066(cpu, 0x31202dc); }
pub fn f_031202e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202e1); } r_03120066(cpu, 0x31202e1); }
pub fn f_031202e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202e6); } r_03120066(cpu, 0x31202e6); }
pub fn f_031202eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202eb); } r_03120066(cpu, 0x31202eb); }
pub fn f_031202f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202f0); } r_03120066(cpu, 0x31202f0); }
pub fn f_031202f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202f5); } r_03120066(cpu, 0x31202f5); }
pub fn f_031202fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202fa); } r_03120066(cpu, 0x31202fa); }
pub fn f_031202ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31202ff); } r_03120066(cpu, 0x31202ff); }
pub fn f_03120304(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120304); } r_03120066(cpu, 0x3120304); }
pub fn f_03120309(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120309); } r_03120066(cpu, 0x3120309); }
pub fn f_0312030e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312030e); } r_03120066(cpu, 0x312030e); }
pub fn f_03120313(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120313); } r_03120066(cpu, 0x3120313); }
pub fn f_03120318(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120318); } r_03120066(cpu, 0x3120318); }
pub fn f_0312031d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312031d); } r_03120066(cpu, 0x312031d); }
pub fn f_03120322(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120322); } r_03120066(cpu, 0x3120322); }
pub fn f_03120327(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120327); } r_03120066(cpu, 0x3120327); }
pub fn f_0312032c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312032c); } r_03120066(cpu, 0x312032c); }
pub fn f_03120331(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120331); } r_03120066(cpu, 0x3120331); }
pub fn f_03120336(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120336); } r_03120066(cpu, 0x3120336); }
pub fn f_0312033b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312033b); } r_03120066(cpu, 0x312033b); }
pub fn f_03120340(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120340); } r_03120066(cpu, 0x3120340); }
pub fn f_03120345(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120345); } r_03120066(cpu, 0x3120345); }
pub fn f_0312034a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312034a); } r_03120066(cpu, 0x312034a); }
pub fn f_0312034f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312034f); } r_03120066(cpu, 0x312034f); }
pub fn f_03120354(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120354); } r_03120066(cpu, 0x3120354); }
pub fn f_03120359(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120359); } r_03120066(cpu, 0x3120359); }
pub fn f_0312035e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312035e); } r_03120066(cpu, 0x312035e); }
pub fn f_03120363(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120363); } r_03120066(cpu, 0x3120363); }
pub fn f_03120368(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120368); } r_03120066(cpu, 0x3120368); }
pub fn f_0312036d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312036d); } r_03120066(cpu, 0x312036d); }
pub fn f_03120372(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120372); } r_03120066(cpu, 0x3120372); }
pub fn f_03120377(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120377); } r_03120066(cpu, 0x3120377); }
pub fn f_0312037c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312037c); } r_03120066(cpu, 0x312037c); }
pub fn f_03120381(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120381); } r_03120066(cpu, 0x3120381); }
pub fn f_03120386(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120386); } r_03120066(cpu, 0x3120386); }
pub fn f_0312038b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312038b); } r_03120066(cpu, 0x312038b); }
pub fn f_03120390(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120390); } r_03120066(cpu, 0x3120390); }
pub fn f_03120395(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120395); } r_03120066(cpu, 0x3120395); }
pub fn f_0312039a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312039a); } r_03120066(cpu, 0x312039a); }
pub fn f_0312039f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312039f); } r_03120066(cpu, 0x312039f); }
pub fn f_031203a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203a4); } r_03120066(cpu, 0x31203a4); }
pub fn f_031203a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203a9); } r_03120066(cpu, 0x31203a9); }
pub fn f_031203ae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203ae); } r_03120066(cpu, 0x31203ae); }
pub fn f_031203b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203b3); } r_03120066(cpu, 0x31203b3); }
pub fn f_031203b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203b8); } r_03120066(cpu, 0x31203b8); }
pub fn f_031203bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203bd); } r_03120066(cpu, 0x31203bd); }
pub fn f_031203c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203c2); } r_03120066(cpu, 0x31203c2); }
pub fn f_031203c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203c7); } r_03120066(cpu, 0x31203c7); }
pub fn f_031203cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203cc); } r_03120066(cpu, 0x31203cc); }
pub fn f_031203d1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203d1); } r_03120066(cpu, 0x31203d1); }
pub fn f_031203d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203d6); } r_03120066(cpu, 0x31203d6); }
pub fn f_031203db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203db); } r_03120066(cpu, 0x31203db); }
pub fn f_031203e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203e0); } r_03120066(cpu, 0x31203e0); }
pub fn f_031203e5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203e5); } r_03120066(cpu, 0x31203e5); }
pub fn f_031203ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203ea); } r_03120066(cpu, 0x31203ea); }
pub fn f_031203ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203ef); } r_03120066(cpu, 0x31203ef); }
pub fn f_031203f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203f4); } r_03120066(cpu, 0x31203f4); }
pub fn f_031203f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203f9); } r_03120066(cpu, 0x31203f9); }
pub fn f_031203fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31203fe); } r_03120066(cpu, 0x31203fe); }
pub fn f_03120403(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120403); } r_03120066(cpu, 0x3120403); }
pub fn f_03120408(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120408); } r_03120066(cpu, 0x3120408); }
pub fn f_0312040d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312040d); } r_03120066(cpu, 0x312040d); }
pub fn f_03120412(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120412); } r_03120066(cpu, 0x3120412); }
pub fn f_03120417(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120417); } r_03120066(cpu, 0x3120417); }
pub fn f_0312041c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312041c); } r_03120066(cpu, 0x312041c); }
pub fn f_03120421(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120421); } r_03120066(cpu, 0x3120421); }
pub fn f_03120426(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120426); } r_03120066(cpu, 0x3120426); }
pub fn f_0312042b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312042b); } r_03120066(cpu, 0x312042b); }
pub fn f_03120430(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120430); } r_03120066(cpu, 0x3120430); }
pub fn f_03120435(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120435); } r_03120066(cpu, 0x3120435); }
pub fn f_0312043a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312043a); } r_03120066(cpu, 0x312043a); }
pub fn f_0312043f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312043f); } r_03120066(cpu, 0x312043f); }
pub fn f_03120444(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120444); } r_03120066(cpu, 0x3120444); }
pub fn f_03120449(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120449); } r_03120066(cpu, 0x3120449); }
pub fn f_0312044e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312044e); } r_03120066(cpu, 0x312044e); }
pub fn f_03120453(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120453); } r_03120066(cpu, 0x3120453); }
pub fn f_03120458(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120458); } r_03120066(cpu, 0x3120458); }
pub fn f_0312045d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312045d); } r_03120066(cpu, 0x312045d); }
pub fn f_03120462(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120462); } r_03120066(cpu, 0x3120462); }
pub fn f_03120467(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120467); } r_03120066(cpu, 0x3120467); }
pub fn f_0312046c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312046c); } r_03120066(cpu, 0x312046c); }
pub fn f_03120471(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120471); } r_03120066(cpu, 0x3120471); }
pub fn f_03120476(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120476); } r_03120066(cpu, 0x3120476); }
pub fn f_0312047b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312047b); } r_03120066(cpu, 0x312047b); }
pub fn f_03120480(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120480); } r_03120066(cpu, 0x3120480); }
pub fn f_03120485(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120485); } r_03120066(cpu, 0x3120485); }
pub fn f_0312048a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312048a); } r_03120066(cpu, 0x312048a); }
pub fn f_0312048f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312048f); } r_03120066(cpu, 0x312048f); }
pub fn f_03120494(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120494); } r_03120066(cpu, 0x3120494); }
pub fn f_03120499(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120499); } r_03120066(cpu, 0x3120499); }
pub fn f_0312049e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312049e); } r_03120066(cpu, 0x312049e); }
pub fn f_031204a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204a3); } r_03120066(cpu, 0x31204a3); }
pub fn f_031204a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204a8); } r_03120066(cpu, 0x31204a8); }
pub fn f_031204ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204ad); } r_03120066(cpu, 0x31204ad); }
pub fn f_031204b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204b2); } r_03120066(cpu, 0x31204b2); }
pub fn f_031204b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204b7); } r_03120066(cpu, 0x31204b7); }
pub fn f_031204bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204bc); } r_03120066(cpu, 0x31204bc); }
pub fn f_031204c1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204c1); } r_03120066(cpu, 0x31204c1); }
pub fn f_031204c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204c6); } r_03120066(cpu, 0x31204c6); }
pub fn f_031204cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204cb); } r_03120066(cpu, 0x31204cb); }
pub fn f_031204d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204d0); } r_03120066(cpu, 0x31204d0); }
pub fn f_031204d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204d5); } r_03120066(cpu, 0x31204d5); }
pub fn f_031204da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204da); } r_03120066(cpu, 0x31204da); }
pub fn f_031204df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204df); } r_03120066(cpu, 0x31204df); }
pub fn f_031204e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204e4); } r_03120066(cpu, 0x31204e4); }
pub fn f_031204e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204e9); } r_03120066(cpu, 0x31204e9); }
pub fn f_031204ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204ee); } r_03120066(cpu, 0x31204ee); }
pub fn f_031204f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204f3); } r_03120066(cpu, 0x31204f3); }
pub fn f_031204f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204f8); } r_03120066(cpu, 0x31204f8); }
pub fn f_031204fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31204fd); } r_03120066(cpu, 0x31204fd); }
pub fn f_03120502(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120502); } r_03120066(cpu, 0x3120502); }
pub fn f_03120507(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120507); } r_03120066(cpu, 0x3120507); }
pub fn f_0312050c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312050c); } r_03120066(cpu, 0x312050c); }
pub fn f_03120511(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120511); } r_03120066(cpu, 0x3120511); }
pub fn f_03120516(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120516); } r_03120066(cpu, 0x3120516); }
pub fn f_0312051b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312051b); } r_03120066(cpu, 0x312051b); }
pub fn f_03120520(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120520); } r_03120066(cpu, 0x3120520); }
pub fn f_03120525(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120525); } r_03120066(cpu, 0x3120525); }
pub fn f_0312052a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312052a); } r_03120066(cpu, 0x312052a); }
pub fn f_0312052f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312052f); } r_03120066(cpu, 0x312052f); }
pub fn f_03120534(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120534); } r_03120066(cpu, 0x3120534); }
pub fn f_03120539(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120539); } r_03120066(cpu, 0x3120539); }
pub fn f_0312053e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312053e); } r_03120066(cpu, 0x312053e); }
pub fn f_03120543(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120543); } r_03120066(cpu, 0x3120543); }
pub fn f_03120548(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120548); } r_03120066(cpu, 0x3120548); }
pub fn f_0312054d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312054d); } r_03120066(cpu, 0x312054d); }
pub fn f_03120552(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120552); } r_03120066(cpu, 0x3120552); }
pub fn f_03120557(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120557); } r_03120066(cpu, 0x3120557); }
pub fn f_0312055c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312055c); } r_03120066(cpu, 0x312055c); }
pub fn f_03120561(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120561); } r_03120066(cpu, 0x3120561); }
pub fn f_03120566(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120566); } r_03120066(cpu, 0x3120566); }
pub fn f_0312056b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312056b); } r_03120066(cpu, 0x312056b); }
pub fn f_03120570(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120570); } r_03120066(cpu, 0x3120570); }
pub fn f_03120575(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120575); } r_03120066(cpu, 0x3120575); }
pub fn f_0312057a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312057a); } r_03120066(cpu, 0x312057a); }
pub fn f_0312057f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312057f); } r_03120066(cpu, 0x312057f); }
pub fn f_03120584(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120584); } r_03120066(cpu, 0x3120584); }
pub fn f_03120589(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120589); } r_03120066(cpu, 0x3120589); }
pub fn f_0312058e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312058e); } r_03120066(cpu, 0x312058e); }
pub fn f_03120593(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120593); } r_03120066(cpu, 0x3120593); }
pub fn f_03120598(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120598); } r_03120066(cpu, 0x3120598); }
pub fn f_0312059d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312059d); } r_03120066(cpu, 0x312059d); }
pub fn f_031205a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205a2); } r_03120066(cpu, 0x31205a2); }
pub fn f_031205a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205a7); } r_03120066(cpu, 0x31205a7); }
pub fn f_031205ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205ac); } r_03120066(cpu, 0x31205ac); }
pub fn f_031205b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205b1); } r_03120066(cpu, 0x31205b1); }
pub fn f_031205b6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205b6); } r_03120066(cpu, 0x31205b6); }
pub fn f_031205bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205bb); } r_03120066(cpu, 0x31205bb); }
pub fn f_031205c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205c0); } r_03120066(cpu, 0x31205c0); }
pub fn f_031205c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205c5); } r_03120066(cpu, 0x31205c5); }
pub fn f_031205ca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205ca); } r_03120066(cpu, 0x31205ca); }
pub fn f_031205cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205cf); } r_03120066(cpu, 0x31205cf); }
pub fn f_031205d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205d4); } r_03120066(cpu, 0x31205d4); }
pub fn f_031205d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205d9); } r_03120066(cpu, 0x31205d9); }
pub fn f_031205de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205de); } r_03120066(cpu, 0x31205de); }
pub fn f_031205e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205e3); } r_03120066(cpu, 0x31205e3); }
pub fn f_031205e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205e8); } r_03120066(cpu, 0x31205e8); }
pub fn f_031205ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205ed); } r_03120066(cpu, 0x31205ed); }
pub fn f_031205f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205f2); } r_03120066(cpu, 0x31205f2); }
pub fn f_031205f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205f7); } r_03120066(cpu, 0x31205f7); }
pub fn f_031205fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31205fc); } r_03120066(cpu, 0x31205fc); }
pub fn f_03120601(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120601); } r_03120066(cpu, 0x3120601); }
pub fn f_03120606(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120606); } r_03120066(cpu, 0x3120606); }
pub fn f_0312060b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312060b); } r_03120066(cpu, 0x312060b); }
pub fn f_03120610(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120610); } r_03120066(cpu, 0x3120610); }
pub fn f_03120615(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120615); } r_03120066(cpu, 0x3120615); }
pub fn f_0312061a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312061a); } r_03120066(cpu, 0x312061a); }
pub fn f_0312061f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312061f); } r_03120066(cpu, 0x312061f); }
pub fn f_03120624(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120624); } r_03120066(cpu, 0x3120624); }
pub fn f_03120629(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120629); } r_03120066(cpu, 0x3120629); }
pub fn f_0312062e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312062e); } r_03120066(cpu, 0x312062e); }
pub fn f_03120633(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120633); } r_03120066(cpu, 0x3120633); }
pub fn f_03120638(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120638); } r_03120066(cpu, 0x3120638); }
pub fn f_0312063d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312063d); } r_03120066(cpu, 0x312063d); }
pub fn f_03120642(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120642); } r_03120066(cpu, 0x3120642); }
pub fn f_03120647(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120647); } r_03120066(cpu, 0x3120647); }
pub fn f_0312064c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312064c); } r_03120066(cpu, 0x312064c); }
pub fn f_03120651(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120651); } r_03120066(cpu, 0x3120651); }
pub fn f_03120656(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120656); } r_03120066(cpu, 0x3120656); }
pub fn f_0312065b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312065b); } r_03120066(cpu, 0x312065b); }
pub fn f_03120660(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120660); } r_03120066(cpu, 0x3120660); }
pub fn f_03120665(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120665); } r_03120066(cpu, 0x3120665); }
pub fn f_0312066a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312066a); } r_03120066(cpu, 0x312066a); }
pub fn f_0312066f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312066f); } r_03120066(cpu, 0x312066f); }
pub fn f_03120674(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120674); } r_03120066(cpu, 0x3120674); }
pub fn f_03120679(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120679); } r_03120066(cpu, 0x3120679); }
pub fn f_0312067e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312067e); } r_03120066(cpu, 0x312067e); }
pub fn f_03120683(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120683); } r_03120066(cpu, 0x3120683); }
pub fn f_03120688(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120688); } r_03120066(cpu, 0x3120688); }
pub fn f_0312068d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312068d); } r_03120066(cpu, 0x312068d); }
pub fn f_03120692(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120692); } r_03120066(cpu, 0x3120692); }
pub fn f_03120697(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120697); } r_03120066(cpu, 0x3120697); }
pub fn f_0312069c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312069c); } r_03120066(cpu, 0x312069c); }
pub fn f_031206a1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206a1); } r_03120066(cpu, 0x31206a1); }
pub fn f_031206a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206a6); } r_03120066(cpu, 0x31206a6); }
pub fn f_031206ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206ab); } r_03120066(cpu, 0x31206ab); }
pub fn f_031206b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206b0); } r_03120066(cpu, 0x31206b0); }
pub fn f_031206b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206b5); } r_03120066(cpu, 0x31206b5); }
pub fn f_031206ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206ba); } r_03120066(cpu, 0x31206ba); }
pub fn f_031206bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206bf); } r_03120066(cpu, 0x31206bf); }
pub fn f_031206c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206c4); } r_03120066(cpu, 0x31206c4); }
pub fn f_031206c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206c9); } r_03120066(cpu, 0x31206c9); }
pub fn f_031206ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206ce); } r_03120066(cpu, 0x31206ce); }
pub fn f_031206d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206d3); } r_03120066(cpu, 0x31206d3); }
pub fn f_031206d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206d8); } r_03120066(cpu, 0x31206d8); }
pub fn f_031206dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206dd); } r_03120066(cpu, 0x31206dd); }
pub fn f_031206e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206e2); } r_03120066(cpu, 0x31206e2); }
pub fn f_031206e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206e7); } r_03120066(cpu, 0x31206e7); }
pub fn f_031206ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206ec); } r_03120066(cpu, 0x31206ec); }
pub fn f_031206f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206f1); } r_03120066(cpu, 0x31206f1); }
pub fn f_031206f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206f6); } r_03120066(cpu, 0x31206f6); }
pub fn f_031206fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31206fb); } r_03120066(cpu, 0x31206fb); }
pub fn f_03120700(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120700); } r_03120066(cpu, 0x3120700); }
pub fn f_03120705(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120705); } r_03120066(cpu, 0x3120705); }
pub fn f_0312070a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312070a); } r_03120066(cpu, 0x312070a); }
pub fn f_0312070f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312070f); } r_03120066(cpu, 0x312070f); }
pub fn f_03120714(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120714); } r_03120066(cpu, 0x3120714); }
pub fn f_03120719(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120719); } r_03120066(cpu, 0x3120719); }
pub fn f_0312071e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312071e); } r_03120066(cpu, 0x312071e); }
pub fn f_03120723(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120723); } r_03120066(cpu, 0x3120723); }
pub fn f_03120728(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120728); } r_03120066(cpu, 0x3120728); }
pub fn f_0312072d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312072d); } r_03120066(cpu, 0x312072d); }
pub fn f_03120732(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120732); } r_03120066(cpu, 0x3120732); }
pub fn f_03120737(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120737); } r_03120066(cpu, 0x3120737); }
pub fn f_0312073c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312073c); } r_03120066(cpu, 0x312073c); }
pub fn f_03120741(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120741); } r_03120066(cpu, 0x3120741); }
pub fn f_03120746(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120746); } r_03120066(cpu, 0x3120746); }
pub fn f_0312074b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312074b); } r_03120066(cpu, 0x312074b); }
pub fn f_03120750(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120750); } r_03120066(cpu, 0x3120750); }
pub fn f_03120755(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120755); } r_03120066(cpu, 0x3120755); }
pub fn f_0312075a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312075a); } r_03120066(cpu, 0x312075a); }
pub fn f_0312075f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312075f); } r_03120066(cpu, 0x312075f); }
pub fn f_03120764(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120764); } r_03120066(cpu, 0x3120764); }
pub fn f_03120769(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120769); } r_03120066(cpu, 0x3120769); }
pub fn f_0312076e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312076e); } r_03120066(cpu, 0x312076e); }
pub fn f_03120773(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120773); } r_03120066(cpu, 0x3120773); }
pub fn f_03120778(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120778); } r_03120066(cpu, 0x3120778); }
pub fn f_0312077d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312077d); } r_03120066(cpu, 0x312077d); }
pub fn f_03120782(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120782); } r_03120066(cpu, 0x3120782); }
pub fn f_03120787(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120787); } r_03120066(cpu, 0x3120787); }
pub fn f_0312078c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312078c); } r_03120066(cpu, 0x312078c); }
pub fn f_03120791(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120791); } r_03120066(cpu, 0x3120791); }
pub fn f_03120796(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120796); } r_03120066(cpu, 0x3120796); }
pub fn f_0312079b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312079b); } r_03120066(cpu, 0x312079b); }
pub fn f_031207a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207a0); } r_03120066(cpu, 0x31207a0); }
pub fn f_031207a5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207a5); } r_03120066(cpu, 0x31207a5); }
pub fn f_031207aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207aa); } r_03120066(cpu, 0x31207aa); }
pub fn f_031207af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207af); } r_03120066(cpu, 0x31207af); }
pub fn f_031207b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207b4); } r_03120066(cpu, 0x31207b4); }
pub fn f_031207b9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207b9); } r_03120066(cpu, 0x31207b9); }
pub fn f_031207be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207be); } r_03120066(cpu, 0x31207be); }
pub fn f_031207c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207c3); } r_03120066(cpu, 0x31207c3); }
pub fn f_031207c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207c8); } r_03120066(cpu, 0x31207c8); }
pub fn f_031207cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207cd); } r_03120066(cpu, 0x31207cd); }
pub fn f_031207d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207d2); } r_03120066(cpu, 0x31207d2); }
pub fn f_031207d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207d7); } r_03120066(cpu, 0x31207d7); }
pub fn f_031207dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207dc); } r_03120066(cpu, 0x31207dc); }
pub fn f_031207e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207e1); } r_03120066(cpu, 0x31207e1); }
pub fn f_031207e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207e6); } r_03120066(cpu, 0x31207e6); }
pub fn f_031207eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207eb); } r_03120066(cpu, 0x31207eb); }
pub fn f_031207f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207f0); } r_03120066(cpu, 0x31207f0); }
pub fn f_031207f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207f5); } r_03120066(cpu, 0x31207f5); }
pub fn f_031207fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207fa); } r_03120066(cpu, 0x31207fa); }
pub fn f_031207ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31207ff); } r_03120066(cpu, 0x31207ff); }
pub fn f_03120804(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120804); } r_03120066(cpu, 0x3120804); }
pub fn f_03120809(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120809); } r_03120066(cpu, 0x3120809); }
pub fn f_0312080e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312080e); } r_03120066(cpu, 0x312080e); }
pub fn f_03120813(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120813); } r_03120066(cpu, 0x3120813); }
pub fn f_03120818(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120818); } r_03120066(cpu, 0x3120818); }
pub fn f_0312081d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312081d); } r_03120066(cpu, 0x312081d); }
pub fn f_03120822(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120822); } r_03120066(cpu, 0x3120822); }
pub fn f_03120827(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120827); } r_03120066(cpu, 0x3120827); }
pub fn f_0312082c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312082c); } r_03120066(cpu, 0x312082c); }
pub fn f_03120831(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120831); } r_03120066(cpu, 0x3120831); }
pub fn f_03120836(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120836); } r_03120066(cpu, 0x3120836); }
pub fn f_0312083b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312083b); } r_03120066(cpu, 0x312083b); }
pub fn f_03120840(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120840); } r_03120066(cpu, 0x3120840); }
pub fn f_03120845(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120845); } r_03120066(cpu, 0x3120845); }
pub fn f_0312084a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312084a); } r_03120066(cpu, 0x312084a); }
pub fn f_0312084f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312084f); } r_03120066(cpu, 0x312084f); }
pub fn f_03120854(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120854); } r_03120066(cpu, 0x3120854); }
pub fn f_03120859(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120859); } r_03120066(cpu, 0x3120859); }
pub fn f_0312085e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312085e); } r_03120066(cpu, 0x312085e); }
pub fn f_03120863(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120863); } r_03120066(cpu, 0x3120863); }
pub fn f_03120868(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120868); } r_03120066(cpu, 0x3120868); }
pub fn f_0312086d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312086d); } r_03120066(cpu, 0x312086d); }
pub fn f_03120872(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120872); } r_03120066(cpu, 0x3120872); }
pub fn f_03120877(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120877); } r_03120066(cpu, 0x3120877); }
pub fn f_0312087c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312087c); } r_03120066(cpu, 0x312087c); }
pub fn f_03120881(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120881); } r_03120066(cpu, 0x3120881); }
pub fn f_03120886(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120886); } r_03120066(cpu, 0x3120886); }
pub fn f_0312088b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312088b); } r_03120066(cpu, 0x312088b); }
pub fn f_03120890(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120890); } r_03120066(cpu, 0x3120890); }
pub fn f_03120895(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120895); } r_03120066(cpu, 0x3120895); }
pub fn f_0312089a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312089a); } r_03120066(cpu, 0x312089a); }
pub fn f_0312089f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312089f); } r_03120066(cpu, 0x312089f); }
pub fn f_031208a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208a4); } r_03120066(cpu, 0x31208a4); }
pub fn f_031208a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208a9); } r_03120066(cpu, 0x31208a9); }
pub fn f_031208ae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208ae); } r_03120066(cpu, 0x31208ae); }
pub fn f_031208b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208b3); } r_03120066(cpu, 0x31208b3); }
pub fn f_031208b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208b8); } r_03120066(cpu, 0x31208b8); }
pub fn f_031208bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208bd); } r_03120066(cpu, 0x31208bd); }
pub fn f_031208c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208c2); } r_03120066(cpu, 0x31208c2); }
pub fn f_031208c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208c7); } r_03120066(cpu, 0x31208c7); }
pub fn f_031208cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208cc); } r_03120066(cpu, 0x31208cc); }
pub fn f_031208d1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208d1); } r_03120066(cpu, 0x31208d1); }
pub fn f_031208d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208d6); } r_03120066(cpu, 0x31208d6); }
pub fn f_031208db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208db); } r_03120066(cpu, 0x31208db); }
pub fn f_031208e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208e0); } r_03120066(cpu, 0x31208e0); }
pub fn f_031208e5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208e5); } r_03120066(cpu, 0x31208e5); }
pub fn f_031208ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208ea); } r_03120066(cpu, 0x31208ea); }
pub fn f_031208ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208ef); } r_03120066(cpu, 0x31208ef); }
pub fn f_031208f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208f4); } r_03120066(cpu, 0x31208f4); }
pub fn f_031208f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208f9); } r_03120066(cpu, 0x31208f9); }
pub fn f_031208fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31208fe); } r_03120066(cpu, 0x31208fe); }
pub fn f_03120903(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120903); } r_03120066(cpu, 0x3120903); }
pub fn f_03120908(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120908); } r_03120066(cpu, 0x3120908); }
pub fn f_0312090d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312090d); } r_03120066(cpu, 0x312090d); }
pub fn f_03120912(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120912); } r_03120066(cpu, 0x3120912); }
pub fn f_03120917(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120917); } r_03120066(cpu, 0x3120917); }
pub fn f_0312091c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312091c); } r_03120066(cpu, 0x312091c); }
pub fn f_03120921(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120921); } r_03120066(cpu, 0x3120921); }
pub fn f_03120926(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120926); } r_03120066(cpu, 0x3120926); }
pub fn f_0312092b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312092b); } r_03120066(cpu, 0x312092b); }
pub fn f_03120930(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120930); } r_03120066(cpu, 0x3120930); }
pub fn f_03120935(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120935); } r_03120066(cpu, 0x3120935); }
pub fn f_0312093a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312093a); } r_03120066(cpu, 0x312093a); }
pub fn f_0312093f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312093f); } r_03120066(cpu, 0x312093f); }
pub fn f_03120944(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120944); } r_03120066(cpu, 0x3120944); }
pub fn f_03120949(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120949); } r_03120066(cpu, 0x3120949); }
pub fn f_0312094e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312094e); } r_03120066(cpu, 0x312094e); }
pub fn f_03120953(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120953); } r_03120066(cpu, 0x3120953); }
pub fn f_03120958(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120958); } r_03120066(cpu, 0x3120958); }
pub fn f_0312095d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312095d); } r_03120066(cpu, 0x312095d); }
pub fn f_03120962(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120962); } r_03120066(cpu, 0x3120962); }
pub fn f_03120967(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120967); } r_03120066(cpu, 0x3120967); }
pub fn f_0312096c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312096c); } r_03120066(cpu, 0x312096c); }
pub fn f_03120971(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120971); } r_03120066(cpu, 0x3120971); }
pub fn f_03120976(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120976); } r_03120066(cpu, 0x3120976); }
pub fn f_0312097b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312097b); } r_03120066(cpu, 0x312097b); }
pub fn f_03120980(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120980); } r_03120066(cpu, 0x3120980); }
pub fn f_03120985(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120985); } r_03120066(cpu, 0x3120985); }
pub fn f_0312098a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312098a); } r_03120066(cpu, 0x312098a); }
pub fn f_0312098f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312098f); } r_03120066(cpu, 0x312098f); }
pub fn f_03120994(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120994); } r_03120066(cpu, 0x3120994); }
pub fn f_03120999(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120999); } r_03120066(cpu, 0x3120999); }
pub fn f_0312099e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x312099e); } r_03120066(cpu, 0x312099e); }
pub fn f_031209a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209a3); } r_03120066(cpu, 0x31209a3); }
pub fn f_031209a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209a8); } r_03120066(cpu, 0x31209a8); }
pub fn f_031209ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209ad); } r_03120066(cpu, 0x31209ad); }
pub fn f_031209b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209b2); } r_03120066(cpu, 0x31209b2); }
pub fn f_031209b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209b7); } r_03120066(cpu, 0x31209b7); }
pub fn f_031209bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209bc); } r_03120066(cpu, 0x31209bc); }
pub fn f_031209c1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209c1); } r_03120066(cpu, 0x31209c1); }
pub fn f_031209c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209c6); } r_03120066(cpu, 0x31209c6); }
pub fn f_031209cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209cb); } r_03120066(cpu, 0x31209cb); }
pub fn f_031209d0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209d0); } r_03120066(cpu, 0x31209d0); }
pub fn f_031209d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209d5); } r_03120066(cpu, 0x31209d5); }
pub fn f_031209da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209da); } r_03120066(cpu, 0x31209da); }
pub fn f_031209df(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209df); } r_03120066(cpu, 0x31209df); }
pub fn f_031209e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209e4); } r_03120066(cpu, 0x31209e4); }
pub fn f_031209e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209e9); } r_03120066(cpu, 0x31209e9); }
pub fn f_031209ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209ee); } r_03120066(cpu, 0x31209ee); }
pub fn f_031209f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209f3); } r_03120066(cpu, 0x31209f3); }
pub fn f_031209f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209f8); } r_03120066(cpu, 0x31209f8); }
pub fn f_031209fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x31209fd); } r_03120066(cpu, 0x31209fd); }
pub fn f_03120a02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a02); } r_03120066(cpu, 0x3120a02); }
pub fn f_03120a07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a07); } r_03120066(cpu, 0x3120a07); }
pub fn f_03120a0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a0c); } r_03120066(cpu, 0x3120a0c); }
pub fn f_03120a11(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a11); } r_03120066(cpu, 0x3120a11); }
pub fn f_03120a16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a16); } r_03120066(cpu, 0x3120a16); }
pub fn f_03120a1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a1b); } r_03120066(cpu, 0x3120a1b); }
pub fn f_03120a20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a20); } r_03120066(cpu, 0x3120a20); }
pub fn f_03120a25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a25); } r_03120066(cpu, 0x3120a25); }
pub fn f_03120a2a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a2a); } r_03120066(cpu, 0x3120a2a); }
pub fn f_03120a2f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a2f); } r_03120066(cpu, 0x3120a2f); }
pub fn f_03120a34(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a34); } r_03120066(cpu, 0x3120a34); }
pub fn f_03120a39(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a39); } r_03120066(cpu, 0x3120a39); }
pub fn f_03120a3e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a3e); } r_03120066(cpu, 0x3120a3e); }
pub fn f_03120a43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a43); } r_03120066(cpu, 0x3120a43); }
pub fn f_03120a48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a48); } r_03120066(cpu, 0x3120a48); }
pub fn f_03120a4d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a4d); } r_03120066(cpu, 0x3120a4d); }
pub fn f_03120a52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a52); } r_03120066(cpu, 0x3120a52); }
pub fn f_03120a57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a57); } r_03120066(cpu, 0x3120a57); }
pub fn f_03120a5c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a5c); } r_03120066(cpu, 0x3120a5c); }
pub fn f_03120a61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a61); } r_03120066(cpu, 0x3120a61); }
pub fn f_03120a66(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a66); } r_03120066(cpu, 0x3120a66); }
pub fn f_03120a6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a6b); } r_03120066(cpu, 0x3120a6b); }
pub fn f_03120a70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a70); } r_03120066(cpu, 0x3120a70); }
pub fn f_03120a75(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a75); } r_03120066(cpu, 0x3120a75); }
pub fn f_03120a7a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a7a); } r_03120066(cpu, 0x3120a7a); }
pub fn f_03120a7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a7f); } r_03120066(cpu, 0x3120a7f); }
pub fn f_03120a84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a84); } r_03120066(cpu, 0x3120a84); }
pub fn f_03120a89(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a89); } r_03120066(cpu, 0x3120a89); }
pub fn f_03120a8e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a8e); } r_03120066(cpu, 0x3120a8e); }
pub fn f_03120a93(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a93); } r_03120066(cpu, 0x3120a93); }
pub fn f_03120a98(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a98); } r_03120066(cpu, 0x3120a98); }
pub fn f_03120a9d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120a9d); } r_03120066(cpu, 0x3120a9d); }
pub fn f_03120aa2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120aa2); } r_03120066(cpu, 0x3120aa2); }
pub fn f_03120aa7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120aa7); } r_03120066(cpu, 0x3120aa7); }
pub fn f_03120aac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120aac); } r_03120066(cpu, 0x3120aac); }
pub fn f_03120ab1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ab1); } r_03120066(cpu, 0x3120ab1); }
pub fn f_03120ab6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ab6); } r_03120066(cpu, 0x3120ab6); }
pub fn f_03120abb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120abb); } r_03120066(cpu, 0x3120abb); }
pub fn f_03120ac0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ac0); } r_03120066(cpu, 0x3120ac0); }
pub fn f_03120ac5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ac5); } r_03120066(cpu, 0x3120ac5); }
pub fn f_03120aca(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120aca); } r_03120066(cpu, 0x3120aca); }
pub fn f_03120acf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120acf); } r_03120066(cpu, 0x3120acf); }
pub fn f_03120ad4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ad4); } r_03120066(cpu, 0x3120ad4); }
pub fn f_03120ad9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ad9); } r_03120066(cpu, 0x3120ad9); }
pub fn f_03120ade(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ade); } r_03120066(cpu, 0x3120ade); }
pub fn f_03120ae3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ae3); } r_03120066(cpu, 0x3120ae3); }
pub fn f_03120ae8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ae8); } r_03120066(cpu, 0x3120ae8); }
pub fn f_03120aed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120aed); } r_03120066(cpu, 0x3120aed); }
pub fn f_03120af2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120af2); } r_03120066(cpu, 0x3120af2); }
pub fn f_03120af7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120af7); } r_03120066(cpu, 0x3120af7); }
pub fn f_03120afc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120afc); } r_03120066(cpu, 0x3120afc); }
pub fn f_03120b01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b01); } r_03120066(cpu, 0x3120b01); }
pub fn f_03120b06(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b06); } r_03120066(cpu, 0x3120b06); }
pub fn f_03120b0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b0b); } r_03120066(cpu, 0x3120b0b); }
pub fn f_03120b10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b10); } r_03120066(cpu, 0x3120b10); }
pub fn f_03120b15(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b15); } r_03120066(cpu, 0x3120b15); }
pub fn f_03120b1a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b1a); } r_03120066(cpu, 0x3120b1a); }
pub fn f_03120b1f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b1f); } r_03120066(cpu, 0x3120b1f); }
pub fn f_03120b24(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b24); } r_03120066(cpu, 0x3120b24); }
pub fn f_03120b29(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b29); } r_03120066(cpu, 0x3120b29); }
pub fn f_03120b2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b2e); } r_03120066(cpu, 0x3120b2e); }
pub fn f_03120b33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b33); } r_03120066(cpu, 0x3120b33); }
pub fn f_03120b38(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b38); } r_03120066(cpu, 0x3120b38); }
pub fn f_03120b3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b3d); } r_03120066(cpu, 0x3120b3d); }
pub fn f_03120b42(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b42); } r_03120066(cpu, 0x3120b42); }
pub fn f_03120b47(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b47); } r_03120066(cpu, 0x3120b47); }
pub fn f_03120b4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b4c); } r_03120066(cpu, 0x3120b4c); }
pub fn f_03120b51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b51); } r_03120066(cpu, 0x3120b51); }
pub fn f_03120b56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b56); } r_03120066(cpu, 0x3120b56); }
pub fn f_03120b5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b5b); } r_03120066(cpu, 0x3120b5b); }
pub fn f_03120b60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b60); } r_03120066(cpu, 0x3120b60); }
pub fn f_03120b65(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b65); } r_03120066(cpu, 0x3120b65); }
pub fn f_03120b6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b6a); } r_03120066(cpu, 0x3120b6a); }
pub fn f_03120b6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b6f); } r_03120066(cpu, 0x3120b6f); }
pub fn f_03120b74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b74); } r_03120066(cpu, 0x3120b74); }
pub fn f_03120b79(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b79); } r_03120066(cpu, 0x3120b79); }
pub fn f_03120b7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b7e); } r_03120066(cpu, 0x3120b7e); }
pub fn f_03120b83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b83); } r_03120066(cpu, 0x3120b83); }
pub fn f_03120b88(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b88); } r_03120066(cpu, 0x3120b88); }
pub fn f_03120b8d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b8d); } r_03120066(cpu, 0x3120b8d); }
pub fn f_03120b92(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b92); } r_03120066(cpu, 0x3120b92); }
pub fn f_03120b97(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b97); } r_03120066(cpu, 0x3120b97); }
pub fn f_03120b9c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120b9c); } r_03120066(cpu, 0x3120b9c); }
pub fn f_03120ba1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ba1); } r_03120066(cpu, 0x3120ba1); }
pub fn f_03120ba6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120ba6); } r_03120066(cpu, 0x3120ba6); }
pub fn f_03120bab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bab); } r_03120066(cpu, 0x3120bab); }
pub fn f_03120bb0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bb0); } r_03120066(cpu, 0x3120bb0); }
pub fn f_03120bb5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bb5); } r_03120066(cpu, 0x3120bb5); }
pub fn f_03120bba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bba); } r_03120066(cpu, 0x3120bba); }
pub fn f_03120bbf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bbf); } r_03120066(cpu, 0x3120bbf); }
pub fn f_03120bc4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bc4); } r_03120066(cpu, 0x3120bc4); }
pub fn f_03120bc9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bc9); } r_03120066(cpu, 0x3120bc9); }
pub fn f_03120bce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bce); } r_03120066(cpu, 0x3120bce); }
pub fn f_03120bd3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bd3); } r_03120066(cpu, 0x3120bd3); }
pub fn f_03120bd8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bd8); } r_03120066(cpu, 0x3120bd8); }
pub fn f_03120bdd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bdd); } r_03120066(cpu, 0x3120bdd); }
pub fn f_03120be2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120be2); } r_03120066(cpu, 0x3120be2); }
pub fn f_03120be7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120be7); } r_03120066(cpu, 0x3120be7); }
pub fn f_03120bec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bec); } r_03120066(cpu, 0x3120bec); }
pub fn f_03120bf1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bf1); } r_03120066(cpu, 0x3120bf1); }
pub fn f_03120bf6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bf6); } r_03120066(cpu, 0x3120bf6); }
pub fn f_03120bfb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120bfb); } r_03120066(cpu, 0x3120bfb); }
pub fn f_03120c00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c00); } r_03120066(cpu, 0x3120c00); }
pub fn f_03120c05(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c05); } r_03120066(cpu, 0x3120c05); }
pub fn f_03120c0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c0a); } r_03120066(cpu, 0x3120c0a); }
pub fn f_03120c0f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c0f); } r_03120066(cpu, 0x3120c0f); }
pub fn f_03120c14(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c14); } r_03120066(cpu, 0x3120c14); }
pub fn f_03120c19(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c19); } r_03120066(cpu, 0x3120c19); }
pub fn f_03120c1e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c1e); } r_03120066(cpu, 0x3120c1e); }
pub fn f_03120c23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3120c23); } r_03120066(cpu, 0x3120c23); }
fn r_03120066(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x3120066 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312006b; continue;
 }
 0x312006b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120070; continue;
 }
 0x3120070 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120075; continue;
 }
 0x3120075 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312007a; continue;
 }
 0x312007a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312007f; continue;
 }
 0x312007f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120084; continue;
 }
 0x3120084 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120089; continue;
 }
 0x3120089 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312008e; continue;
 }
 0x312008e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120093; continue;
 }
 0x3120093 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120098; continue;
 }
 0x3120098 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312009d; continue;
 }
 0x312009d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200a2; continue;
 }
 0x31200a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200a7; continue;
 }
 0x31200a7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200ac; continue;
 }
 0x31200ac => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200b1; continue;
 }
 0x31200b1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200b6; continue;
 }
 0x31200b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200bb; continue;
 }
 0x31200bb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200c0; continue;
 }
 0x31200c0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200c5; continue;
 }
 0x31200c5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200ca; continue;
 }
 0x31200ca => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200cf; continue;
 }
 0x31200cf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200d4; continue;
 }
 0x31200d4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200d9; continue;
 }
 0x31200d9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200de; continue;
 }
 0x31200de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200e3; continue;
 }
 0x31200e3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200e8; continue;
 }
 0x31200e8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200ed; continue;
 }
 0x31200ed => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200f2; continue;
 }
 0x31200f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200f7; continue;
 }
 0x31200f7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31200fc; continue;
 }
 0x31200fc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120101; continue;
 }
 0x3120101 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120106; continue;
 }
 0x3120106 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312010b; continue;
 }
 0x312010b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120110; continue;
 }
 0x3120110 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120115; continue;
 }
 0x3120115 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312011a; continue;
 }
 0x312011a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312011f; continue;
 }
 0x312011f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120124; continue;
 }
 0x3120124 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120129; continue;
 }
 0x3120129 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312012e; continue;
 }
 0x312012e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120133; continue;
 }
 0x3120133 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120138; continue;
 }
 0x3120138 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312013d; continue;
 }
 0x312013d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120142; continue;
 }
 0x3120142 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120147; continue;
 }
 0x3120147 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312014c; continue;
 }
 0x312014c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120151; continue;
 }
 0x3120151 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120156; continue;
 }
 0x3120156 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312015b; continue;
 }
 0x312015b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120160; continue;
 }
 0x3120160 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120165; continue;
 }
 0x3120165 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312016a; continue;
 }
 0x312016a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312016f; continue;
 }
 0x312016f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120174; continue;
 }
 0x3120174 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120179; continue;
 }
 0x3120179 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312017e; continue;
 }
 0x312017e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120183; continue;
 }
 0x3120183 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120188; continue;
 }
 0x3120188 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312018d; continue;
 }
 0x312018d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120192; continue;
 }
 0x3120192 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120197; continue;
 }
 0x3120197 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312019c; continue;
 }
 0x312019c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201a1; continue;
 }
 0x31201a1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201a6; continue;
 }
 0x31201a6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201ab; continue;
 }
 0x31201ab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201b0; continue;
 }
 0x31201b0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201b5; continue;
 }
 0x31201b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201ba; continue;
 }
 0x31201ba => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201bf; continue;
 }
 0x31201bf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201c4; continue;
 }
 0x31201c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201c9; continue;
 }
 0x31201c9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201ce; continue;
 }
 0x31201ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201d3; continue;
 }
 0x31201d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201d8; continue;
 }
 0x31201d8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201dd; continue;
 }
 0x31201dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201e2; continue;
 }
 0x31201e2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201e7; continue;
 }
 0x31201e7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201ec; continue;
 }
 0x31201ec => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201f1; continue;
 }
 0x31201f1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201f6; continue;
 }
 0x31201f6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31201fb; continue;
 }
 0x31201fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120200; continue;
 }
 0x3120200 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120205; continue;
 }
 0x3120205 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312020a; continue;
 }
 0x312020a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312020f; continue;
 }
 0x312020f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120214; continue;
 }
 0x3120214 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120219; continue;
 }
 0x3120219 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312021e; continue;
 }
 0x312021e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120223; continue;
 }
 0x3120223 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120228; continue;
 }
 0x3120228 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312022d; continue;
 }
 0x312022d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120232; continue;
 }
 0x3120232 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120237; continue;
 }
 0x3120237 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312023c; continue;
 }
 0x312023c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120241; continue;
 }
 0x3120241 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120246; continue;
 }
 0x3120246 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312024b; continue;
 }
 0x312024b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120250; continue;
 }
 0x3120250 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120255; continue;
 }
 0x3120255 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312025a; continue;
 }
 0x312025a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312025f; continue;
 }
 0x312025f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120264; continue;
 }
 0x3120264 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120269; continue;
 }
 0x3120269 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312026e; continue;
 }
 0x312026e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120273; continue;
 }
 0x3120273 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120278; continue;
 }
 0x3120278 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312027d; continue;
 }
 0x312027d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120282; continue;
 }
 0x3120282 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120287; continue;
 }
 0x3120287 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312028c; continue;
 }
 0x312028c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120291; continue;
 }
 0x3120291 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120296; continue;
 }
 0x3120296 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312029b; continue;
 }
 0x312029b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202a0; continue;
 }
 0x31202a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202a5; continue;
 }
 0x31202a5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202aa; continue;
 }
 0x31202aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202af; continue;
 }
 0x31202af => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202b4; continue;
 }
 0x31202b4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202b9; continue;
 }
 0x31202b9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202be; continue;
 }
 0x31202be => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202c3; continue;
 }
 0x31202c3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202c8; continue;
 }
 0x31202c8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202cd; continue;
 }
 0x31202cd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202d2; continue;
 }
 0x31202d2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202d7; continue;
 }
 0x31202d7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202dc; continue;
 }
 0x31202dc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202e1; continue;
 }
 0x31202e1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202e6; continue;
 }
 0x31202e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202eb; continue;
 }
 0x31202eb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202f0; continue;
 }
 0x31202f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202f5; continue;
 }
 0x31202f5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202fa; continue;
 }
 0x31202fa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31202ff; continue;
 }
 0x31202ff => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120304; continue;
 }
 0x3120304 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120309; continue;
 }
 0x3120309 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312030e; continue;
 }
 0x312030e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120313; continue;
 }
 0x3120313 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120318; continue;
 }
 0x3120318 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312031d; continue;
 }
 0x312031d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120322; continue;
 }
 0x3120322 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120327; continue;
 }
 0x3120327 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312032c; continue;
 }
 0x312032c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120331; continue;
 }
 0x3120331 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120336; continue;
 }
 0x3120336 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312033b; continue;
 }
 0x312033b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120340; continue;
 }
 0x3120340 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120345; continue;
 }
 0x3120345 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312034a; continue;
 }
 0x312034a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312034f; continue;
 }
 0x312034f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120354; continue;
 }
 0x3120354 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120359; continue;
 }
 0x3120359 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312035e; continue;
 }
 0x312035e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120363; continue;
 }
 0x3120363 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120368; continue;
 }
 0x3120368 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312036d; continue;
 }
 0x312036d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120372; continue;
 }
 0x3120372 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120377; continue;
 }
 0x3120377 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312037c; continue;
 }
 0x312037c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120381; continue;
 }
 0x3120381 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120386; continue;
 }
 0x3120386 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312038b; continue;
 }
 0x312038b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120390; continue;
 }
 0x3120390 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120395; continue;
 }
 0x3120395 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312039a; continue;
 }
 0x312039a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312039f; continue;
 }
 0x312039f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203a4; continue;
 }
 0x31203a4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203a9; continue;
 }
 0x31203a9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203ae; continue;
 }
 0x31203ae => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203b3; continue;
 }
 0x31203b3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203b8; continue;
 }
 0x31203b8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203bd; continue;
 }
 0x31203bd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203c2; continue;
 }
 0x31203c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203c7; continue;
 }
 0x31203c7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203cc; continue;
 }
 0x31203cc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203d1; continue;
 }
 0x31203d1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203d6; continue;
 }
 0x31203d6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203db; continue;
 }
 0x31203db => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203e0; continue;
 }
 0x31203e0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203e5; continue;
 }
 0x31203e5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203ea; continue;
 }
 0x31203ea => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203ef; continue;
 }
 0x31203ef => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203f4; continue;
 }
 0x31203f4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203f9; continue;
 }
 0x31203f9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31203fe; continue;
 }
 0x31203fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120403; continue;
 }
 0x3120403 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120408; continue;
 }
 0x3120408 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312040d; continue;
 }
 0x312040d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120412; continue;
 }
 0x3120412 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120417; continue;
 }
 0x3120417 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312041c; continue;
 }
 0x312041c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120421; continue;
 }
 0x3120421 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120426; continue;
 }
 0x3120426 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312042b; continue;
 }
 0x312042b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120430; continue;
 }
 0x3120430 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120435; continue;
 }
 0x3120435 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312043a; continue;
 }
 0x312043a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312043f; continue;
 }
 0x312043f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120444; continue;
 }
 0x3120444 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120449; continue;
 }
 0x3120449 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312044e; continue;
 }
 0x312044e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120453; continue;
 }
 0x3120453 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120458; continue;
 }
 0x3120458 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312045d; continue;
 }
 0x312045d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120462; continue;
 }
 0x3120462 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120467; continue;
 }
 0x3120467 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312046c; continue;
 }
 0x312046c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120471; continue;
 }
 0x3120471 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120476; continue;
 }
 0x3120476 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312047b; continue;
 }
 0x312047b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120480; continue;
 }
 0x3120480 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120485; continue;
 }
 0x3120485 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312048a; continue;
 }
 0x312048a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312048f; continue;
 }
 0x312048f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120494; continue;
 }
 0x3120494 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120499; continue;
 }
 0x3120499 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312049e; continue;
 }
 0x312049e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204a3; continue;
 }
 0x31204a3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204a8; continue;
 }
 0x31204a8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204ad; continue;
 }
 0x31204ad => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204b2; continue;
 }
 0x31204b2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204b7; continue;
 }
 0x31204b7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204bc; continue;
 }
 0x31204bc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204c1; continue;
 }
 0x31204c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204c6; continue;
 }
 0x31204c6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204cb; continue;
 }
 0x31204cb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204d0; continue;
 }
 0x31204d0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204d5; continue;
 }
 0x31204d5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204da; continue;
 }
 0x31204da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204df; continue;
 }
 0x31204df => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204e4; continue;
 }
 0x31204e4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204e9; continue;
 }
 0x31204e9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204ee; continue;
 }
 0x31204ee => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204f3; continue;
 }
 0x31204f3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204f8; continue;
 }
 0x31204f8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31204fd; continue;
 }
 0x31204fd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120502; continue;
 }
 0x3120502 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120507; continue;
 }
 0x3120507 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312050c; continue;
 }
 0x312050c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120511; continue;
 }
 0x3120511 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120516; continue;
 }
 0x3120516 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312051b; continue;
 }
 0x312051b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120520; continue;
 }
 0x3120520 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120525; continue;
 }
 0x3120525 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312052a; continue;
 }
 0x312052a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312052f; continue;
 }
 0x312052f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120534; continue;
 }
 0x3120534 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120539; continue;
 }
 0x3120539 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312053e; continue;
 }
 0x312053e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120543; continue;
 }
 0x3120543 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120548; continue;
 }
 0x3120548 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312054d; continue;
 }
 0x312054d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120552; continue;
 }
 0x3120552 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120557; continue;
 }
 0x3120557 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312055c; continue;
 }
 0x312055c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120561; continue;
 }
 0x3120561 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120566; continue;
 }
 0x3120566 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312056b; continue;
 }
 0x312056b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120570; continue;
 }
 0x3120570 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120575; continue;
 }
 0x3120575 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312057a; continue;
 }
 0x312057a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312057f; continue;
 }
 0x312057f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120584; continue;
 }
 0x3120584 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120589; continue;
 }
 0x3120589 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312058e; continue;
 }
 0x312058e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120593; continue;
 }
 0x3120593 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120598; continue;
 }
 0x3120598 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312059d; continue;
 }
 0x312059d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205a2; continue;
 }
 0x31205a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205a7; continue;
 }
 0x31205a7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205ac; continue;
 }
 0x31205ac => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205b1; continue;
 }
 0x31205b1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205b6; continue;
 }
 0x31205b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205bb; continue;
 }
 0x31205bb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205c0; continue;
 }
 0x31205c0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205c5; continue;
 }
 0x31205c5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205ca; continue;
 }
 0x31205ca => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205cf; continue;
 }
 0x31205cf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205d4; continue;
 }
 0x31205d4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205d9; continue;
 }
 0x31205d9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205de; continue;
 }
 0x31205de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205e3; continue;
 }
 0x31205e3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205e8; continue;
 }
 0x31205e8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205ed; continue;
 }
 0x31205ed => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205f2; continue;
 }
 0x31205f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205f7; continue;
 }
 0x31205f7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31205fc; continue;
 }
 0x31205fc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120601; continue;
 }
 0x3120601 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120606; continue;
 }
 0x3120606 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312060b; continue;
 }
 0x312060b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120610; continue;
 }
 0x3120610 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120615; continue;
 }
 0x3120615 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312061a; continue;
 }
 0x312061a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312061f; continue;
 }
 0x312061f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120624; continue;
 }
 0x3120624 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120629; continue;
 }
 0x3120629 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312062e; continue;
 }
 0x312062e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120633; continue;
 }
 0x3120633 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120638; continue;
 }
 0x3120638 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312063d; continue;
 }
 0x312063d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120642; continue;
 }
 0x3120642 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120647; continue;
 }
 0x3120647 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312064c; continue;
 }
 0x312064c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120651; continue;
 }
 0x3120651 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120656; continue;
 }
 0x3120656 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312065b; continue;
 }
 0x312065b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120660; continue;
 }
 0x3120660 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120665; continue;
 }
 0x3120665 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312066a; continue;
 }
 0x312066a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312066f; continue;
 }
 0x312066f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120674; continue;
 }
 0x3120674 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120679; continue;
 }
 0x3120679 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312067e; continue;
 }
 0x312067e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120683; continue;
 }
 0x3120683 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120688; continue;
 }
 0x3120688 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312068d; continue;
 }
 0x312068d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120692; continue;
 }
 0x3120692 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120697; continue;
 }
 0x3120697 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312069c; continue;
 }
 0x312069c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206a1; continue;
 }
 0x31206a1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206a6; continue;
 }
 0x31206a6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206ab; continue;
 }
 0x31206ab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206b0; continue;
 }
 0x31206b0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206b5; continue;
 }
 0x31206b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206ba; continue;
 }
 0x31206ba => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206bf; continue;
 }
 0x31206bf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206c4; continue;
 }
 0x31206c4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206c9; continue;
 }
 0x31206c9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206ce; continue;
 }
 0x31206ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206d3; continue;
 }
 0x31206d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206d8; continue;
 }
 0x31206d8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206dd; continue;
 }
 0x31206dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206e2; continue;
 }
 0x31206e2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206e7; continue;
 }
 0x31206e7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206ec; continue;
 }
 0x31206ec => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206f1; continue;
 }
 0x31206f1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206f6; continue;
 }
 0x31206f6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31206fb; continue;
 }
 0x31206fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120700; continue;
 }
 0x3120700 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120705; continue;
 }
 0x3120705 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312070a; continue;
 }
 0x312070a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312070f; continue;
 }
 0x312070f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120714; continue;
 }
 0x3120714 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120719; continue;
 }
 0x3120719 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312071e; continue;
 }
 0x312071e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120723; continue;
 }
 0x3120723 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120728; continue;
 }
 0x3120728 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312072d; continue;
 }
 0x312072d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120732; continue;
 }
 0x3120732 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120737; continue;
 }
 0x3120737 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312073c; continue;
 }
 0x312073c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120741; continue;
 }
 0x3120741 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120746; continue;
 }
 0x3120746 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312074b; continue;
 }
 0x312074b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120750; continue;
 }
 0x3120750 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120755; continue;
 }
 0x3120755 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312075a; continue;
 }
 0x312075a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312075f; continue;
 }
 0x312075f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120764; continue;
 }
 0x3120764 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120769; continue;
 }
 0x3120769 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312076e; continue;
 }
 0x312076e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120773; continue;
 }
 0x3120773 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120778; continue;
 }
 0x3120778 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312077d; continue;
 }
 0x312077d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120782; continue;
 }
 0x3120782 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120787; continue;
 }
 0x3120787 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312078c; continue;
 }
 0x312078c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120791; continue;
 }
 0x3120791 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120796; continue;
 }
 0x3120796 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312079b; continue;
 }
 0x312079b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207a0; continue;
 }
 0x31207a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207a5; continue;
 }
 0x31207a5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207aa; continue;
 }
 0x31207aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207af; continue;
 }
 0x31207af => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207b4; continue;
 }
 0x31207b4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207b9; continue;
 }
 0x31207b9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207be; continue;
 }
 0x31207be => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207c3; continue;
 }
 0x31207c3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207c8; continue;
 }
 0x31207c8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207cd; continue;
 }
 0x31207cd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207d2; continue;
 }
 0x31207d2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207d7; continue;
 }
 0x31207d7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207dc; continue;
 }
 0x31207dc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207e1; continue;
 }
 0x31207e1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207e6; continue;
 }
 0x31207e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207eb; continue;
 }
 0x31207eb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207f0; continue;
 }
 0x31207f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207f5; continue;
 }
 0x31207f5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207fa; continue;
 }
 0x31207fa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31207ff; continue;
 }
 0x31207ff => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120804; continue;
 }
 0x3120804 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120809; continue;
 }
 0x3120809 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312080e; continue;
 }
 0x312080e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120813; continue;
 }
 0x3120813 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120818; continue;
 }
 0x3120818 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312081d; continue;
 }
 0x312081d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120822; continue;
 }
 0x3120822 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120827; continue;
 }
 0x3120827 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312082c; continue;
 }
 0x312082c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120831; continue;
 }
 0x3120831 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120836; continue;
 }
 0x3120836 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312083b; continue;
 }
 0x312083b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120840; continue;
 }
 0x3120840 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120845; continue;
 }
 0x3120845 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312084a; continue;
 }
 0x312084a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312084f; continue;
 }
 0x312084f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120854; continue;
 }
 0x3120854 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120859; continue;
 }
 0x3120859 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312085e; continue;
 }
 0x312085e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120863; continue;
 }
 0x3120863 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120868; continue;
 }
 0x3120868 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312086d; continue;
 }
 0x312086d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120872; continue;
 }
 0x3120872 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120877; continue;
 }
 0x3120877 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312087c; continue;
 }
 0x312087c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120881; continue;
 }
 0x3120881 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120886; continue;
 }
 0x3120886 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312088b; continue;
 }
 0x312088b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120890; continue;
 }
 0x3120890 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120895; continue;
 }
 0x3120895 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312089a; continue;
 }
 0x312089a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312089f; continue;
 }
 0x312089f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208a4; continue;
 }
 0x31208a4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208a9; continue;
 }
 0x31208a9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208ae; continue;
 }
 0x31208ae => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208b3; continue;
 }
 0x31208b3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208b8; continue;
 }
 0x31208b8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208bd; continue;
 }
 0x31208bd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208c2; continue;
 }
 0x31208c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208c7; continue;
 }
 0x31208c7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208cc; continue;
 }
 0x31208cc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208d1; continue;
 }
 0x31208d1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208d6; continue;
 }
 0x31208d6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208db; continue;
 }
 0x31208db => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208e0; continue;
 }
 0x31208e0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208e5; continue;
 }
 0x31208e5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208ea; continue;
 }
 0x31208ea => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208ef; continue;
 }
 0x31208ef => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208f4; continue;
 }
 0x31208f4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208f9; continue;
 }
 0x31208f9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31208fe; continue;
 }
 0x31208fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120903; continue;
 }
 0x3120903 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120908; continue;
 }
 0x3120908 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312090d; continue;
 }
 0x312090d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120912; continue;
 }
 0x3120912 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120917; continue;
 }
 0x3120917 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312091c; continue;
 }
 0x312091c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120921; continue;
 }
 0x3120921 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120926; continue;
 }
 0x3120926 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312092b; continue;
 }
 0x312092b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120930; continue;
 }
 0x3120930 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120935; continue;
 }
 0x3120935 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312093a; continue;
 }
 0x312093a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312093f; continue;
 }
 0x312093f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120944; continue;
 }
 0x3120944 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120949; continue;
 }
 0x3120949 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312094e; continue;
 }
 0x312094e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120953; continue;
 }
 0x3120953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120958; continue;
 }
 0x3120958 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312095d; continue;
 }
 0x312095d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120962; continue;
 }
 0x3120962 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120967; continue;
 }
 0x3120967 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312096c; continue;
 }
 0x312096c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120971; continue;
 }
 0x3120971 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120976; continue;
 }
 0x3120976 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312097b; continue;
 }
 0x312097b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120980; continue;
 }
 0x3120980 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120985; continue;
 }
 0x3120985 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312098a; continue;
 }
 0x312098a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312098f; continue;
 }
 0x312098f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120994; continue;
 }
 0x3120994 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120999; continue;
 }
 0x3120999 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x312099e; continue;
 }
 0x312099e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209a3; continue;
 }
 0x31209a3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209a8; continue;
 }
 0x31209a8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209ad; continue;
 }
 0x31209ad => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209b2; continue;
 }
 0x31209b2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209b7; continue;
 }
 0x31209b7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209bc; continue;
 }
 0x31209bc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209c1; continue;
 }
 0x31209c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209c6; continue;
 }
 0x31209c6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209cb; continue;
 }
 0x31209cb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209d0; continue;
 }
 0x31209d0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209d5; continue;
 }
 0x31209d5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209da; continue;
 }
 0x31209da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209df; continue;
 }
 0x31209df => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209e4; continue;
 }
 0x31209e4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209e9; continue;
 }
 0x31209e9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209ee; continue;
 }
 0x31209ee => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209f3; continue;
 }
 0x31209f3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209f8; continue;
 }
 0x31209f8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x31209fd; continue;
 }
 0x31209fd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a02; continue;
 }
 0x3120a02 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a07; continue;
 }
 0x3120a07 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a0c; continue;
 }
 0x3120a0c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a11; continue;
 }
 0x3120a11 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a16; continue;
 }
 0x3120a16 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a1b; continue;
 }
 0x3120a1b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a20; continue;
 }
 0x3120a20 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a25; continue;
 }
 0x3120a25 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a2a; continue;
 }
 0x3120a2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a2f; continue;
 }
 0x3120a2f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a34; continue;
 }
 0x3120a34 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a39; continue;
 }
 0x3120a39 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a3e; continue;
 }
 0x3120a3e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a43; continue;
 }
 0x3120a43 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a48; continue;
 }
 0x3120a48 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a4d; continue;
 }
 0x3120a4d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a52; continue;
 }
 0x3120a52 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a57; continue;
 }
 0x3120a57 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a5c; continue;
 }
 0x3120a5c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a61; continue;
 }
 0x3120a61 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a66; continue;
 }
 0x3120a66 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a6b; continue;
 }
 0x3120a6b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a70; continue;
 }
 0x3120a70 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a75; continue;
 }
 0x3120a75 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a7a; continue;
 }
 0x3120a7a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a7f; continue;
 }
 0x3120a7f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a84; continue;
 }
 0x3120a84 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a89; continue;
 }
 0x3120a89 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a8e; continue;
 }
 0x3120a8e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a93; continue;
 }
 0x3120a93 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a98; continue;
 }
 0x3120a98 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120a9d; continue;
 }
 0x3120a9d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120aa2; continue;
 }
 0x3120aa2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120aa7; continue;
 }
 0x3120aa7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120aac; continue;
 }
 0x3120aac => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ab1; continue;
 }
 0x3120ab1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ab6; continue;
 }
 0x3120ab6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120abb; continue;
 }
 0x3120abb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ac0; continue;
 }
 0x3120ac0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ac5; continue;
 }
 0x3120ac5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120aca; continue;
 }
 0x3120aca => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120acf; continue;
 }
 0x3120acf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ad4; continue;
 }
 0x3120ad4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ad9; continue;
 }
 0x3120ad9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ade; continue;
 }
 0x3120ade => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ae3; continue;
 }
 0x3120ae3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ae8; continue;
 }
 0x3120ae8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120aed; continue;
 }
 0x3120aed => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120af2; continue;
 }
 0x3120af2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120af7; continue;
 }
 0x3120af7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120afc; continue;
 }
 0x3120afc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b01; continue;
 }
 0x3120b01 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b06; continue;
 }
 0x3120b06 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b0b; continue;
 }
 0x3120b0b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b10; continue;
 }
 0x3120b10 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b15; continue;
 }
 0x3120b15 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b1a; continue;
 }
 0x3120b1a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b1f; continue;
 }
 0x3120b1f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b24; continue;
 }
 0x3120b24 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b29; continue;
 }
 0x3120b29 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b2e; continue;
 }
 0x3120b2e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b33; continue;
 }
 0x3120b33 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b38; continue;
 }
 0x3120b38 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b3d; continue;
 }
 0x3120b3d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b42; continue;
 }
 0x3120b42 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b47; continue;
 }
 0x3120b47 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b4c; continue;
 }
 0x3120b4c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b51; continue;
 }
 0x3120b51 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b56; continue;
 }
 0x3120b56 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b5b; continue;
 }
 0x3120b5b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b60; continue;
 }
 0x3120b60 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b65; continue;
 }
 0x3120b65 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b6a; continue;
 }
 0x3120b6a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b6f; continue;
 }
 0x3120b6f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b74; continue;
 }
 0x3120b74 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b79; continue;
 }
 0x3120b79 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b7e; continue;
 }
 0x3120b7e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b83; continue;
 }
 0x3120b83 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b88; continue;
 }
 0x3120b88 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b8d; continue;
 }
 0x3120b8d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b92; continue;
 }
 0x3120b92 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b97; continue;
 }
 0x3120b97 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120b9c; continue;
 }
 0x3120b9c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ba1; continue;
 }
 0x3120ba1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120ba6; continue;
 }
 0x3120ba6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bab; continue;
 }
 0x3120bab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bb0; continue;
 }
 0x3120bb0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bb5; continue;
 }
 0x3120bb5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bba; continue;
 }
 0x3120bba => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bbf; continue;
 }
 0x3120bbf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bc4; continue;
 }
 0x3120bc4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bc9; continue;
 }
 0x3120bc9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bce; continue;
 }
 0x3120bce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bd3; continue;
 }
 0x3120bd3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bd8; continue;
 }
 0x3120bd8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bdd; continue;
 }
 0x3120bdd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120be2; continue;
 }
 0x3120be2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120be7; continue;
 }
 0x3120be7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bec; continue;
 }
 0x3120bec => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bf1; continue;
 }
 0x3120bf1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bf6; continue;
 }
 0x3120bf6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120bfb; continue;
 }
 0x3120bfb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c00; continue;
 }
 0x3120c00 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c05; continue;
 }
 0x3120c05 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c0a; continue;
 }
 0x3120c0a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c0f; continue;
 }
 0x3120c0f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c14; continue;
 }
 0x3120c14 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c19; continue;
 }
 0x3120c19 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x3120c1e; continue;
 }
 0x3120c1e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, cpu.edx); cpu.edi = v; }
  pc = 0x3120c23; continue;
 }
 0x3120c23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
