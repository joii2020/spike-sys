#![no_main]
use lazy_static::lazy_static;
use libfuzzer_sys::fuzz_target;
use rvv_encode::encode;
use std::collections::HashSet;
use std::str;

#[rustfmt::skip]
const COMPLETED_RVV_INST_VEC: [&str; 286] = [
    "vsetivli",
    "vsetvli",
    "vsetvl",
    "vlm.v",
    "vsm.v",
    "vle8.v",
    "vle16.v",
    "vle32.v",
    "vle64.v",
    "vle128.v",
    "vle256.v",
    "vle512.v",
    "vle1024.v",
    "vse8.v",
    "vse16.v",
    "vse32.v",
    "vse64.v",
    "vse128.v",
    "vse256.v",
    "vse512.v",
    "vse1024.v",
    "vadd.vv",
    "vadd.vx",
    "vadd.vi",
    "vsub.vv",
    "vsub.vx",
    "vrsub.vx",
    "vrsub.vi",
    "vwaddu.vv",
    "vwaddu.vx",
    "vwsubu.vv",
    "vwsubu.vx",
    "vwadd.vv",
    "vwadd.vx",
    "vwsub.vv",
    "vwsub.vx",
    "vwaddu.wv",
    "vwaddu.wx",
    "vwsubu.wv",
    "vwsubu.wx",
    "vwadd.wv",
    "vwadd.wx",
    "vwsub.wv",
    "vwsub.wx",
    "vzext.vf8",
    "vsext.vf8",
    "vzext.vf4",
    "vsext.vf4",
    "vzext.vf2",
    "vsext.vf2",
    "vadc.vvm",
    "vadc.vxm",
    "vadc.vim",
    "vmadc.vvm",
    "vmadc.vxm",
    "vmadc.vim",
    "vmadc.vv",
    "vmadc.vx",
    "vmadc.vi",
    "vsbc.vvm",
    "vsbc.vxm",
    "vmsbc.vvm",
    "vmsbc.vxm",
    "vmsbc.vv",
    "vmsbc.vx",
    "vand.vv",
    "vand.vi",
    "vand.vx",
    "vor.vv",
    "vor.vx",
    "vor.vi",
    "vxor.vv",
    "vxor.vx",
    "vxor.vi",
    "vsll.vv",
    "vsll.vx",
    "vsll.vi",
    "vsrl.vv",
    "vsrl.vx",
    "vsrl.vi",
    "vsra.vv",
    "vsra.vx",
    "vsra.vi",
    "vnsrl.wv",
    "vnsrl.wx",
    "vnsrl.wi",
    "vnsra.wv",
    "vnsra.wx",
    "vnsra.wi",
    "vmseq.vv",
    "vmseq.vx",
    "vmseq.vi",
    "vmsne.vv",
    "vmsne.vx",
    "vmsne.vi",
    "vmsltu.vv",
    "vmsltu.vx",
    "vmslt.vv",
    "vmslt.vx",
    "vmsleu.vv",
    "vmsleu.vx",
    "vmsleu.vi",
    "vmsle.vv",
    "vmsle.vx",
    "vmsle.vi",
    "vmsgtu.vx",
    "vmsgtu.vi",
    "vmsgt.vx",
    "vmsgt.vi",
    "vminu.vv",
    "vminu.vx",
    "vmin.vv",
    "vmin.vx",
    "vmaxu.vv",
    "vmaxu.vx",
    "vmax.vv",
    "vmax.vx",
    "vmul.vv",
    "vmul.vx",
    "vmulh.vv",
    "vmulh.vx",
    "vmulhu.vv",
    "vmulhu.vx",
    "vmulhsu.vv",
    "vmulhsu.vx",
    "vdivu.vv",
    "vdivu.vx",
    "vdiv.vv",
    "vdiv.vx",
    "vremu.vv",
    "vremu.vx",
    "vrem.vv",
    "vrem.vx",
    "vwmulu.vv",
    "vwmulu.vx",
    "vwmulsu.vv",
    "vwmulsu.vx",
    "vwmul.vv",
    "vwmul.vx",
    "vmv.v.v",
    "vmv.v.x",
    "vmv.v.i",
    "vsaddu.vv",
    "vsaddu.vx",
    "vsaddu.vi",
    "vsadd.vv",
    "vsadd.vx",
    "vsadd.vi",
    "vssubu.vv",
    "vssubu.vx",
    "vssub.vv",
    "vssub.vx",
    "vaaddu.vv",
    "vaaddu.vx",
    "vaadd.vv",
    "vaadd.vx",
    "vasubu.vv",
    "vasubu.vx",
    "vasub.vv",
    "vasub.vx",
    "vfirst.m",
    "vmv1r.v",
    "vmv2r.v",
    "vmv4r.v",
    "vmv8r.v",
    "vlse8.v",
    "vlse16.v",
    "vlse32.v",
    "vlse64.v",
    "vlse128.v",
    "vlse256.v",
    "vlse512.v",
    "vlse1024.v",
    "vsse8.v",
    "vsse16.v",
    "vsse32.v",
    "vsse64.v",
    "vsse128.v",
    "vsse256.v",
    "vsse512.v",
    "vsse1024.v",
    "vluxei8.v",
    "vluxei16.v",
    "vluxei32.v",
    "vluxei64.v",
    "vsuxei8.v",
    "vsuxei16.v",
    "vsuxei32.v",
    "vsuxei64.v",
    "vloxei8.v",
    "vloxei16.v",
    "vloxei32.v",
    "vloxei64.v",
    "vsoxei8.v",
    "vsoxei16.v",
    "vsoxei32.v",
    "vsoxei64.v",
    "vl1re8.v",
    "vl1re16.v",
    "vl1re32.v",
    "vl1re64.v",
    "vl2re8.v",
    "vl2re16.v",
    "vl2re32.v",
    "vl2re64.v",
    "vl4re8.v",
    "vl4re16.v",
    "vl4re32.v",
    "vl4re64.v",
    "vl8re8.v",
    "vl8re16.v",
    "vl8re32.v",
    "vl8re64.v",
    "vs1r.v",
    "vs2r.v",
    "vs4r.v",
    "vs8r.v",
    "vmacc.vv",
    "vmacc.vx",
    "vnmsac.vv",
    "vnmsac.vx",
    "vmadd.vv",
    "vmadd.vx",
    "vnmsub.vv",
    "vnmsub.vx",
    "vwmaccu.vv",
    "vwmaccu.vx",
    "vwmacc.vv",
    "vwmacc.vx",
    "vwmaccsu.vv",
    "vwmaccsu.vx",
    "vwmaccus.vx",
    "vmerge.vvm",
    "vmerge.vxm",
    "vmerge.vim",
    "vsmul.vv",
    "vsmul.vx",
    "vssrl.vx",
    "vssrl.vv",
    "vssrl.vi",
    "vssra.vv",
    "vssra.vx",
    "vssra.vi",
    "vnclipu.wv",
    "vnclipu.wx",
    "vnclipu.wi",
    "vnclip.wv",
    "vnclip.wx",
    "vnclip.wi",
    "vredsum.vs",
    "vredand.vs",
    "vredor.vs",
    "vredxor.vs",
    "vredminu.vs",
    "vredmin.vs",
    "vredmaxu.vs",
    "vredmax.vs",
    "vwredsumu.vs",
    "vwredsum.vs",
    "vmand.mm",
    "vmnand.mm",
    "vmandnot.mm",
    "vmxor.mm",
    "vmor.mm",
    "vmnor.mm",
    "vmornot.mm",
    "vmxnor.mm",
    "vcpop.m",
    "vmsbf.m",
    "vmsof.m",
    "vmsif.m",
    "viota.m",
    "vid.v",
    "vmv.x.s",
    "vmv.s.x",
    "vcompress.vm",
    "vslide1up.vx",
    "vslideup.vx",
    "vslideup.vi",
    "vslide1down.vx",
    "vslidedown.vx",
    "vslidedown.vi",
    "vrgather.vx",
    "vrgather.vv",
    "vrgatherei16.vv",
    "vrgather.vi",
];

lazy_static! {
    static ref COMPLETED_RVV_INST: HashSet<&'static str> = HashSet::from(COMPLETED_RVV_INST_VEC);
}

fn check_asm(inst: u32, xlen: u32) {
    let dis = unsafe { spike_sys::rvv_new_disassembler(xlen) };

    let mut output = [0u8; 64];
    let mut output_len = output.len() as u32;
    let ret = unsafe { spike_sys::rvv_disassemble(dis, inst as u64, output.as_mut_ptr(), &mut output_len) };
    unsafe {
        spike_sys::rvv_delete_disassembler(dis);
    }

    if ret != 0 {
        return;
    }
    let asm_str = str::from_utf8(&output[0..output_len as usize]).ok().unwrap();
    if asm_str == "unknown" {
        return;
    }

    let inst_name = {
        let i = asm_str.find(" ");
        if i.is_none() {
            Option::None
        } else {
            Option::Some(&asm_str[0..i.unwrap()])
        }
    };
    if inst_name.is_none() {
        return;
    }

    if COMPLETED_RVV_INST.get(inst_name.unwrap()).is_none() {
        return;
    }

    let inst2 = encode(asm_str, false);
    assert!(inst2.is_ok(), "Encode error: inst: 0x{:x?}, asm: {}", inst, asm_str);

    let inst2 = inst2.ok();
    assert!(
        inst2.is_some(),
        "Encode error: inst: 0x{:x?}, asm: {}, inst2: none",
        inst,
        asm_str
    );

    let inst2 = inst2.unwrap();
    assert!(
        inst2.is_some(),
        "Encode error: inst: 0x{:x?}, asm: {}, inst2: none",
        inst,
        asm_str
    );

    let inst2 = inst2.unwrap();
    assert!(
        inst == inst2,
        "Error: inst: 0x{:x?}, rvv_inst: 0x{:x?}, asm: {}",
        inst,
        inst2,
        asm_str
    );
}

// fn main() {
//     check_asm(0xff0a2457, 0);
// }

fuzz_target!(|data: [u8; 4]| {
    let inst = u32::from_le_bytes(data);
    check_asm(inst, 0);
});
