/* automatically generated by rust-bindgen 0.69.4 */

pub const ENABLE_ALU: u32 = 1;
pub const ENABLE_COMPLEX_X2: u32 = 1;
pub const ENABLE_SSE3: u32 = 1;
pub const ENABLE_ENV_VARS: u32 = 1;
pub const ENABLE_OCL_MEM_GUARDS: u32 = 1;
pub const ENABLE_OPENCL: u32 = 1;
pub const ENABLE_PTHREAD: u32 = 1;
pub const ENABLE_QBDT_CPU_PARALLEL: u32 = 1;
pub const ENABLE_QBDT: u32 = 1;
pub const SEED_DEVRAND: u32 = 1;
pub const ENABLE_QUNIT_CPU_PARALLEL: u32 = 1;
pub const FPPOW: u32 = 5;
pub const PSTRIDEPOW: u32 = 11;
pub const QBCAPPOW: u32 = 6;
pub const UINTPOW: u32 = 6;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub type uintq = ::std::os::raw::c_ulonglong;
pub type IdCallback = ::std::option::Option<unsafe extern "C" fn(arg1: uintq)>;
pub type ProbAmpCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: usize, arg2: f64, arg3: f64) -> bool>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _QrackTimeEvolveOpHeader {
    _unused: [u8; 0],
}
extern "C" {
    pub fn get_error(sid: uintq) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn init_count_type(
        q: uintq,
        tn: bool,
        md: bool,
        sd: bool,
        sh: bool,
        bdt: bool,
        pg: bool,
        zxf: bool,
        hy: bool,
        oc: bool,
        dm: bool,
    ) -> uintq;
}
extern "C" {
    pub fn init_count(q: uintq, dm: bool) -> uintq;
}
extern "C" {
    pub fn init_count_pager(q: uintq, dm: bool) -> uintq;
}
extern "C" {
    pub fn init() -> uintq;
}
extern "C" {
    pub fn init_clone(sid: uintq) -> uintq;
}
extern "C" {
    pub fn destroy(sid: uintq);
}
extern "C" {
    pub fn seed(sid: uintq, s: uintq);
}
extern "C" {
    pub fn set_concurrency(sid: uintq, p: uintq);
}
extern "C" {
    pub fn qstabilizer_out_to_file(sid: uintq, f: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn qstabilizer_in_from_file(sid: uintq, f: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Prob(sid: uintq, q: uintq) -> f64;
}
extern "C" {
    pub fn ProbRdm(sid: uintq, q: uintq) -> f64;
}
extern "C" {
    pub fn PermutationProb(sid: uintq, n: uintq, q: *mut uintq, c: *mut bool) -> f64;
}
extern "C" {
    pub fn PermutationProbRdm(sid: uintq, n: uintq, q: *mut uintq, c: *mut bool, r: bool) -> f64;
}
extern "C" {
    pub fn PermutationExpectation(sid: uintq, n: uintq, q: *mut uintq) -> f64;
}
extern "C" {
    pub fn PermutationExpectationRdm(sid: uintq, n: uintq, q: *mut uintq, r: bool) -> f64;
}
extern "C" {
    pub fn FactorizedExpectation(
        sid: uintq,
        n: uintq,
        q: *mut uintq,
        m: uintq,
        c: *mut uintq,
    ) -> f64;
}
extern "C" {
    pub fn FactorizedExpectationRdm(
        sid: uintq,
        n: uintq,
        q: *mut uintq,
        m: uintq,
        c: *mut uintq,
        r: bool,
    ) -> f64;
}
extern "C" {
    pub fn FactorizedExpectationFp(sid: uintq, n: uintq, q: *mut uintq, c: *mut f32) -> f64;
}
extern "C" {
    pub fn FactorizedExpectationFpRdm(
        sid: uintq,
        n: uintq,
        q: *mut uintq,
        c: *mut f32,
        r: bool,
    ) -> f64;
}
extern "C" {
    pub fn DumpIds(sid: uintq, callback: IdCallback);
}
extern "C" {
    pub fn Dump(sid: uintq, callback: ProbAmpCallback);
}
extern "C" {
    pub fn InKet(sid: uintq, ket: *mut f32);
}
extern "C" {
    pub fn OutKet(sid: uintq, ket: *mut f32);
}
extern "C" {
    pub fn random_choice(sid: uintq, n: usize, p: *mut f64) -> usize;
}
extern "C" {
    pub fn PhaseParity(sid: uintq, lambda: f64, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn JointEnsembleProbability(
        sid: uintq,
        n: uintq,
        b: *mut ::std::os::raw::c_int,
        q: *mut uintq,
    ) -> f64;
}
extern "C" {
    pub fn ResetAll(sid: uintq);
}
extern "C" {
    pub fn allocateQubit(sid: uintq, qid: uintq);
}
extern "C" {
    pub fn release(sid: uintq, q: uintq) -> bool;
}
extern "C" {
    pub fn num_qubits(sid: uintq) -> uintq;
}
extern "C" {
    pub fn X(sid: uintq, q: uintq);
}
extern "C" {
    pub fn Y(sid: uintq, q: uintq);
}
extern "C" {
    pub fn Z(sid: uintq, q: uintq);
}
extern "C" {
    pub fn H(sid: uintq, q: uintq);
}
extern "C" {
    pub fn S(sid: uintq, q: uintq);
}
extern "C" {
    pub fn T(sid: uintq, q: uintq);
}
extern "C" {
    pub fn AdjS(sid: uintq, q: uintq);
}
extern "C" {
    pub fn AdjT(sid: uintq, q: uintq);
}
extern "C" {
    pub fn U(sid: uintq, q: uintq, theta: f64, phi: f64, lambda: f64);
}
extern "C" {
    pub fn Mtrx(sid: uintq, m: *mut f64, q: uintq);
}
extern "C" {
    pub fn MCX(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCY(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCZ(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCH(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCS(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCT(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCAdjS(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCAdjT(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MCU(sid: uintq, n: uintq, c: *mut uintq, q: uintq, theta: f64, phi: f64, lambda: f64);
}
extern "C" {
    pub fn MCMtrx(sid: uintq, n: uintq, c: *mut uintq, m: *mut f64, q: uintq);
}
extern "C" {
    pub fn MACX(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACY(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACZ(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACH(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACS(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACT(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACAdjS(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACAdjT(sid: uintq, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn MACU(sid: uintq, n: uintq, c: *mut uintq, q: uintq, theta: f64, phi: f64, lambda: f64);
}
extern "C" {
    pub fn MACMtrx(sid: uintq, n: uintq, c: *mut uintq, m: *mut f64, q: uintq);
}
extern "C" {
    pub fn UCMtrx(sid: uintq, n: uintq, c: *mut uintq, m: *mut f64, q: uintq, p: uintq);
}
extern "C" {
    pub fn Multiplex1Mtrx(sid: uintq, n: uintq, c: *mut uintq, q: uintq, m: *mut f64);
}
extern "C" {
    pub fn MX(sid: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn MY(sid: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn MZ(sid: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn R(sid: uintq, b: uintq, phi: f64, q: uintq);
}
extern "C" {
    pub fn MCR(sid: uintq, b: uintq, phi: f64, n: uintq, c: *mut uintq, q: uintq);
}
extern "C" {
    pub fn Exp(sid: uintq, n: uintq, b: *mut ::std::os::raw::c_int, phi: f64, q: *mut uintq);
}
extern "C" {
    pub fn MCExp(
        sid: uintq,
        n: uintq,
        b: *mut ::std::os::raw::c_int,
        phi: f64,
        nc: uintq,
        cs: *mut uintq,
        q: *mut uintq,
    );
}
extern "C" {
    pub fn M(sid: uintq, q: uintq) -> uintq;
}
extern "C" {
    pub fn ForceM(sid: uintq, q: uintq, r: bool) -> uintq;
}
extern "C" {
    pub fn MAll(sid: uintq) -> uintq;
}
extern "C" {
    pub fn Measure(sid: uintq, n: uintq, b: *mut ::std::os::raw::c_int, q: *mut uintq) -> uintq;
}
extern "C" {
    pub fn MeasureShots(sid: uintq, n: uintq, q: *mut uintq, s: uintq, m: *mut uintq);
}
extern "C" {
    pub fn SWAP(sid: uintq, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn ISWAP(sid: uintq, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn AdjISWAP(sid: uintq, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn FSim(sid: uintq, theta: f64, phi: f64, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn CSWAP(sid: uintq, n: uintq, c: *mut uintq, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn ACSWAP(sid: uintq, n: uintq, c: *mut uintq, qi1: uintq, qi2: uintq);
}
extern "C" {
    pub fn Compose(sid1: uintq, sid2: uintq, q: *mut uintq);
}
extern "C" {
    pub fn Decompose(sid: uintq, n: uintq, q: *mut uintq) -> uintq;
}
extern "C" {
    pub fn Dispose(sid: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn AND(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn OR(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn XOR(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn NAND(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn NOR(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn XNOR(sid: uintq, qi1: uintq, qi2: uintq, qo: uintq);
}
extern "C" {
    pub fn CLAND(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn CLOR(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn CLXOR(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn CLNAND(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn CLNOR(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn CLXNOR(sid: uintq, ci: bool, qi: uintq, qo: uintq);
}
extern "C" {
    pub fn QFT(sid: uintq, n: uintq, c: *mut uintq);
}
extern "C" {
    pub fn IQFT(sid: uintq, n: uintq, c: *mut uintq);
}
extern "C" {
    pub fn ADD(sid: uintq, na: uintq, a: *mut uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn SUB(sid: uintq, na: uintq, a: *mut uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn ADDS(sid: uintq, na: uintq, a: *mut uintq, s: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn SUBS(sid: uintq, na: uintq, a: *mut uintq, s: uintq, n: uintq, q: *mut uintq);
}
extern "C" {
    pub fn MCADD(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        nq: uintq,
        q: *mut uintq,
    );
}
extern "C" {
    pub fn MCSUB(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        nq: uintq,
        q: *mut uintq,
    );
}
extern "C" {
    pub fn MUL(sid: uintq, na: uintq, a: *mut uintq, n: uintq, q: *mut uintq, o: *mut uintq);
}
extern "C" {
    pub fn DIV(sid: uintq, na: uintq, a: *mut uintq, n: uintq, q: *mut uintq, o: *mut uintq);
}
extern "C" {
    pub fn MULN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn DIVN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn POWN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn MCMUL(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn MCDIV(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn MCMULN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn MCDIVN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn MCPOWN(
        sid: uintq,
        na: uintq,
        a: *mut uintq,
        nc: uintq,
        c: *mut uintq,
        m: *mut uintq,
        n: uintq,
        q: *mut uintq,
        o: *mut uintq,
    );
}
extern "C" {
    pub fn LDA(
        sid: uintq,
        ni: uintq,
        qi: *mut uintq,
        nv: uintq,
        qv: *mut uintq,
        t: *mut ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn ADC(
        sid: uintq,
        s: uintq,
        ni: uintq,
        qi: *mut uintq,
        nv: uintq,
        qv: *mut uintq,
        t: *mut ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn SBC(
        sid: uintq,
        s: uintq,
        ni: uintq,
        qi: *mut uintq,
        nv: uintq,
        qv: *mut uintq,
        t: *mut ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn Hash(sid: uintq, n: uintq, q: *mut uintq, t: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn TrySeparate1Qb(sid: uintq, qi1: uintq) -> bool;
}
extern "C" {
    pub fn TrySeparate2Qb(sid: uintq, qi1: uintq, qi2: uintq) -> bool;
}
extern "C" {
    pub fn TrySeparateTol(sid: uintq, n: uintq, q: *mut uintq, tol: f64) -> bool;
}
extern "C" {
    pub fn GetUnitaryFidelity(sid: uintq) -> f64;
}
extern "C" {
    pub fn ResetUnitaryFidelity(sid: uintq);
}
extern "C" {
    pub fn SetSdrp(sid: uintq, sdrp: f64);
}
extern "C" {
    pub fn SetReactiveSeparate(sid: uintq, irs: bool);
}
extern "C" {
    pub fn SetTInjection(sid: uintq, iti: bool);
}
extern "C" {
    pub fn TimeEvolve(
        sid: uintq,
        t: f64,
        n: uintq,
        teos: *mut _QrackTimeEvolveOpHeader,
        mn: uintq,
        mtrx: *mut f64,
    );
}
extern "C" {
    pub fn init_qneuron(
        sid: uintq,
        n: uintq,
        c: *mut uintq,
        q: uintq,
        f: uintq,
        a: f64,
        tol: f64,
    ) -> uintq;
}
extern "C" {
    pub fn clone_qneuron(nid: uintq) -> uintq;
}
extern "C" {
    pub fn destroy_qneuron(nid: uintq);
}
extern "C" {
    pub fn set_qneuron_angles(nid: uintq, angles: *mut f32);
}
extern "C" {
    pub fn get_qneuron_angles(nid: uintq, angles: *mut f32);
}
extern "C" {
    pub fn set_qneuron_alpha(nid: uintq, alpha: f64);
}
extern "C" {
    pub fn get_qneuron_alpha(nid: uintq) -> f64;
}
extern "C" {
    pub fn set_qneuron_activation_fn(nid: uintq, f: uintq);
}
extern "C" {
    pub fn get_qneuron_activation_fn(nid: uintq) -> uintq;
}
extern "C" {
    pub fn qneuron_predict(nid: uintq, e: bool, r: bool) -> f64;
}
extern "C" {
    pub fn qneuron_unpredict(nid: uintq, e: bool) -> f64;
}
extern "C" {
    pub fn qneuron_learn_cycle(nid: uintq, e: bool) -> f64;
}
extern "C" {
    pub fn qneuron_learn(nid: uintq, eta: f64, e: bool, r: bool);
}
extern "C" {
    pub fn qneuron_learn_permutation(nid: uintq, eta: f64, e: bool, r: bool);
}
extern "C" {
    pub fn init_qcircuit(collapse: bool) -> uintq;
}
extern "C" {
    pub fn init_qcircuit_clone(cid: uintq) -> uintq;
}
extern "C" {
    pub fn qcircuit_inverse(cid: uintq) -> uintq;
}
extern "C" {
    pub fn qcircuit_past_light_cone(cid: uintq, n: uintq, q: *mut uintq) -> uintq;
}
extern "C" {
    pub fn destroy_qcircuit(cid: uintq);
}
extern "C" {
    pub fn get_qcircuit_qubit_count(cid: uintq) -> uintq;
}
extern "C" {
    pub fn qcircuit_swap(cid: uintq, q1: uintq, q2: uintq);
}
extern "C" {
    pub fn qcircuit_append_1qb(cid: uintq, m: *mut f64, q: uintq);
}
extern "C" {
    pub fn qcircuit_append_mc(cid: uintq, m: *mut f64, n: uintq, c: *mut uintq, q: uintq, p: uintq);
}
extern "C" {
    pub fn qcircuit_run(cid: uintq, sid: uintq);
}
extern "C" {
    pub fn qcircuit_out_to_file(cid: uintq, f: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn qcircuit_in_from_file(cid: uintq, f: *mut ::std::os::raw::c_char);
}
