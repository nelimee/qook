// (C) Daniel Strano and the Qrack contributors 2017-2023. All rights reserved.
//
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

use pauli::Pauli;
use qrack_error::QrackError;
use qrack_system;

pub struct QrackSimulator {
    // Interface for all the QRack functionality.
    //
    // Attributes:
    //     sid(i64): Corresponding simulator id.
    sid: u64,
}

impl Clone for QrackSimulator {
    fn clone(&self) -> Self {
        let sid;
        unsafe {
            sid = qrack_system::init_clone(self.sid);
        }
        Self { sid }
    }
}

impl Drop for QrackSimulator {
    fn drop(&mut self) {
        unsafe {
            qrack_system::destroy(self.sid);
        }
    }
}

impl QrackSimulator {
    // constructors
    pub fn new(qubit_count: u64) -> Result<Self, QrackError> {
        let sid;
        unsafe {
            sid = qrack_system::init_count(qubit_count, false);
            if qrack_system::get_error(sid) != 0 {
                return Err(QrackError {});
            }
        }
        return Ok(Self { sid });
    }
    pub fn new_layers(
        qubit_count: u64,
        is_tensor_network: bool,
        is_schmidt_decompose_multi: bool,
        is_schmidt_decompose: bool,
        is_stabilizer_hybrid: bool,
        is_binary_decision_tree: bool,
        is_paged: bool,
        is_cpu_gpu_hybrid: bool,
        is_opencl: bool,
        is_host_pointer: bool,
    ) -> Result<Self, QrackError> {
        let sid;
        if is_tensor_network
            && is_schmidt_decompose
            && is_stabilizer_hybrid
            && !is_binary_decision_tree
            && is_paged
            && is_cpu_gpu_hybrid
            && is_opencl
        {
            if is_schmidt_decompose_multi {
                unsafe {
                    sid = qrack_system::init_count(qubit_count, is_host_pointer);
                }
            } else {
                unsafe {
                    sid = qrack_system::init_count_pager(qubit_count, is_host_pointer);
                }
            }
        } else {
            unsafe {
                sid = qrack_system::init_count_type(
                    qubit_count,
                    is_tensor_network,
                    is_schmidt_decompose_multi,
                    is_schmidt_decompose,
                    is_stabilizer_hybrid,
                    is_binary_decision_tree,
                    is_paged,
                    false,
                    is_cpu_gpu_hybrid,
                    is_opencl,
                    is_host_pointer,
                );
            }
        }

        unsafe {
            if qrack_system::get_error(sid) != 0 {
                return Err(QrackError {});
            }
        }
        return Ok(Self { sid });
    }

    // non-quantum
    pub fn get_error(&self) -> i32 {
        unsafe { qrack_system::get_error(self.sid) }
    }

    pub fn check_error(&self) -> Result<(), QrackError> {
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        return Ok(());
    }

    pub fn get_sid(&self) -> u64 {
        return self.sid;
    }

    pub fn seed(&self, s: u64) -> Result<(), QrackError> {
        unsafe {
            qrack_system::seed(self.sid, s as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn set_concurrency(&self, p: u64) -> Result<(), QrackError> {
        unsafe {
            qrack_system::set_concurrency(self.sid, p as qrack_system::uintq);
        }
        self.check_error()
    }

    // standard gates

    // single-qubits gates
    pub fn x(&self, q: u64) -> Result<(), QrackError> {
        // Applies X gate.
        //
        // Applies the Pauli “X” operator to the qubit at position “q.”
        // The Pauli “X” operator is equivalent to a logical “NOT.”
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::X(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn y(&self, q: u64) -> Result<(), QrackError> {
        // Applies Y gate.
        //
        // Applies the Pauli “Y” operator to the qubit at “q.”
        // The Pauli “Y” operator is equivalent to a logical “NOT" with
        // permutation phase.
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::Y(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn z(&self, q: u64) -> Result<(), QrackError> {
        // Applies Z gate.
        //
        // Applies the Pauli “Z” operator to the qubit at “q.”
        // The Pauli “Z” operator flips the phase of `|1>`
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::Z(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn h(&self, q: u64) -> Result<(), QrackError> {
        // Applies H gate.
        //
        // Applies the Hadarmard operator to the qubit at “q.”
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::H(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn s(&self, q: u64) -> Result<(), QrackError> {
        // Applies S gate.
        //
        // Applies the 1/4 phase rotation to the qubit at “q.”
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::S(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn t(&self, q: u64) -> Result<(), QrackError> {
        // Applies T gate.
        //
        // Applies the 1/8 phase rotation to the qubit at “q.”
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::T(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn adjs(&self, q: u64) -> Result<(), QrackError> {
        // Adjoint of S gate
        //
        // Applies the gate equivalent to the inverse of S gate.
        //
        // Args:
        //     q(64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::AdjS(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn adjt(&self, q: u64) -> Result<(), QrackError> {
        // Adjoint of T gate
        //
        // Applies the gate equivalent to the inverse of T gate.
        //
        // Args:
        //     q(64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::AdjT(self.sid, q as qrack_system::uintq);
        }
        self.check_error()
    }

    pub fn u(&self, q: u64, th: f64, ph: f64, la: f64) -> Result<(), QrackError> {
        // General unitary gate.
        //
        // Applies a gate guaranteed to be unitary.
        // Spans all possible single bit unitary gates.
        //
        // `U(theta, phi, lambda) = RZ(phi + pi/2)RX(theta)RZ(lambda - pi/2)`
        //
        // Args:
        //     q(u64): the qubit number on which the gate is applied to.
        //     th(f64): theta
        //     ph(f64): phi
        //     la(f64): lambda
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::U(self.sid, q, th, ph, la);
        }
        self.check_error()
    }

    pub fn mtrx(&self, m: &[f64; 8], q: u64) -> Result<(), QrackError> {
        // Operation from matrix.
        //
        // Applies arbitrary operation defined by the given matrix.
        //
        // Args:
        //     m(&[f64;8]): row-major complex list representing the operator.
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _m = [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7]];
        unsafe {
            qrack_system::Mtrx(self.sid, _m.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn r(&self, b: Pauli, ph: f64, q: u64) -> Result<(), QrackError> {
        // Rotation gate.
        //
        // Rotate the qubit along the given pauli basis by the given angle.
        //
        // Args:
        //     b(Pauli): Pauli basis
        //     ph(f64): rotation angle
        //     q(u64): the qubit number on which the gate is applied to
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::R(self.sid, b as u64, ph, q);
        }
        self.check_error()
    }

    pub fn exp(&self, b: Vec<Pauli>, ph: f64, q: Vec<u64>) -> Result<(), QrackError> {
        // Arbitrary exponentiation
        //
        // `exp(b, theta) = e^{i*theta*[b_0 . b_1 ...]}`
        // where `.` is the tensor product.
        //
        // Args:
        //     b(Vec<Pauli>): Pauli basis
        //     ph(f64): coefficient of exponentiation
        //     q(Vec<u64>): the qubit number on which the gate is applied to
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if b.len() != q.len() {
            return Err(QrackError {});
        }
        let mut _b = b.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::Exp(
                self.sid,
                _b.len() as u64,
                _b.as_mut_ptr() as *mut i32,
                ph,
                _q.as_mut_ptr() as *mut u64,
            );
        }
        self.check_error()
    }

    // multi-qubit gates

    pub fn mcx(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled X gate
        //
        // If all controlled qubits are `|1>` then the target qubit is flipped.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCX(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcy(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled Y gate
        //
        // If all controlled qubits are `|1>` then the Pauli "Y" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCY(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcz(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled Z gate
        //
        // If all controlled qubits are `|1>` then the Pauli "Z" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCZ(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mch(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled H gate
        //
        // If all controlled qubits are `|1>` then the Hadarmard gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCH(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcs(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled S gate
        //
        // If all controlled qubits are `|1>` then the "S" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCS(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mct(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled T gate
        //
        // If all controlled qubits are `|1>` then the "T" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCT(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcadjs(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled adjs gate
        //
        // If all controlled qubits are `|1>` then the adjs gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCAdjS(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcadjt(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled adjt gate
        //
        // If all controlled qubits are `|1>` then the adjt gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCAdjT(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcu(&self, c: Vec<u64>, q: u64, th: f64, ph: f64, la: f64) -> Result<(), QrackError> {
        // Multi-controlled arbitraty unitary
        //
        // If all controlled qubits are `|1>` then the unitary gate described by
        // parameters is applied to the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //     th(f64): theta
        //     ph(f64): phi
        //     la(f64): lambda
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCU(self.sid, _c.len() as u64, _c.as_mut_ptr(), q, th, ph, la);
        }
        self.check_error()
    }

    pub fn mcmtrx(&self, c: Vec<u64>, m: &[f64; 8], q: u64) -> Result<(), QrackError> {
        // Multi-controlled arbitraty operator
        //
        // If all controlled qubits are `|1>` then the arbitrary operation by
        // parameters is applied to the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits
        //     m(&[f64;8]): row-major complex list representing the operator.
        //     q(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _m = [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7]];
        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCMtrx(
                self.sid,
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                q,
            );
        }
        self.check_error()
    }

    pub fn macx(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled X gate
        //
        // If all controlled qubits are `|0>` then the target qubit is flipped.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACX(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macy(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled Y gate
        //
        // If all controlled qubits are `|0>` then the Pauli "Y" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACY(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macz(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled Z gate
        //
        // If all controlled qubits are `|0>` then the Pauli "Z" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACZ(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mach(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled H gate
        //
        // If all controlled qubits are `|0>` then the Hadarmard gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACH(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macs(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled S gate
        //
        // If all controlled qubits are `|0>` then the "S" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACS(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mact(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled T gate
        //
        // If all controlled qubits are `|0>` then the "T" gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACT(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macadjs(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled adjs gate
        //
        // If all controlled qubits are `|0>` then the adjs gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACAdjS(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macadjt(&self, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled adjt gate
        //
        // If all controlled qubits are `|0>` then the adjt gate is applied to
        // the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACAdjT(self.sid, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn macu(&self, c: Vec<u64>, q: u64, th: f64, ph: f64, la: f64) -> Result<(), QrackError> {
        // Anti multi-controlled arbitraty unitary
        //
        // If all controlled qubits are `|0>` then the unitary gate described by
        // parameters is applied to the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //     th(f64): theta
        //     ph(f64): phi
        //     la(f64): lambda
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACU(self.sid, _c.len() as u64, _c.as_mut_ptr(), q, th, ph, la);
        }
        self.check_error()
    }

    pub fn macmtrx(&self, c: Vec<u64>, m: &[f64; 8], q: u64) -> Result<(), QrackError> {
        // Anti multi-controlled arbitraty operator
        //
        // If all controlled qubits are `|0>` then the arbitrary operation by
        // parameters is applied to the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits
        //     m(&[f64;8]): row-major complex list representing the operator.
        //     q(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _m = [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7]];
        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MACMtrx(
                self.sid,
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                q,
            );
        }
        self.check_error()
    }

    pub fn ucmtrx(&self, c: Vec<u64>, m: &[f64; 8], q: u64, p: u64) -> Result<(), QrackError> {
        // Multi-controlled arbitrary operator with arbitrary controls
        //
        // If all control qubits match 'p' permutation by bit order, then the arbitrary
        // operation by parameters is applied to the target qubit.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits
        //     m(&[f64;8]): row-major complex list representing the operator.
        //     q(u64): target qubit
        //     p(u64): permutation of list of control qubits
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _m = [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7]];
        let mut _c = c.to_vec();
        unsafe {
            qrack_system::UCMtrx(
                self.sid,
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                q,
                p,
            );
        }
        self.check_error()
    }

    pub fn multiplex1_mtrx(&self, c: Vec<u64>, q: u64, m: Vec<f64>) -> Result<(), QrackError> {
        // Multiplex gate
        //
        // A multiplex gate with a single target and an arbitrary number of
        // controls.
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): target qubit.
        //     m(Vec<u64>): row-major complex matrix which defines the operator.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        let mut _m = m.to_vec();
        unsafe {
            qrack_system::Multiplex1Mtrx(
                self.sid,
                _c.len() as u64,
                _c.as_mut_ptr(),
                q,
                _m.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mx(&self, q: Vec<u64>) -> Result<(), QrackError> {
        // Multi X-gate
        //
        // Applies the Pauli “X” operator on all qubits.
        //
        // Args:
        //     q(Vec<u64>): list of qubits to apply X on.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MX(self.sid, _q.len() as u64, _q.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn my(&self, q: Vec<u64>) -> Result<(), QrackError> {
        // Multi Y-gate
        //
        // Applies the Pauli “Y” operator on all qubits.
        //
        // Args:
        //     q(Vec<u64>): list of qubits to apply Y on.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MY(self.sid, _q.len() as u64, _q.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn mz(&self, q: Vec<u64>) -> Result<(), QrackError> {
        // Multi Z-gate
        //
        // Applies the Pauli “Z” operator on all qubits.
        //
        // Args:
        //     q(Vec<u64>): list of qubits to apply Z on.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MZ(self.sid, _q.len() as u64, _q.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn mcr(&self, b: Pauli, ph: f64, c: Vec<u64>, q: u64) -> Result<(), QrackError> {
        // Multi-controlled arbitrary rotation.
        //
        // If all controlled qubits are `|1>` then the arbitrary rotation by
        // parameters is applied to the target qubit.
        //
        // Args:
        //     b(Pauli): Pauli basis
        //     ph(f64): coefficient of exponentiation.
        //     c(Vec<u64>): list of controlled qubits.
        //     q(u64): the qubit number on which the gate is applied to.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::MCR(self.sid, b as u64, ph, _c.len() as u64, _c.as_mut_ptr(), q);
        }
        self.check_error()
    }

    pub fn mcexp(
        &self,
        b: Vec<Pauli>,
        ph: f64,
        cs: Vec<u64>,
        q: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Arbitrary exponentiation
        //
        // `exp(b, theta) = e^{i*theta*[b_0 . b_1 ...]}`
        // where `.` is the tensor product.
        //
        // Args:
        //     b(Vec<Pauli>): Pauli basis
        //     ph(f64): coefficient of exponentiation
        //     q(Vec<u64>): the qubit number on which the gate is applied to
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if b.len() != q.len() {
            return Err(QrackError {});
        }
        let mut _b = b.to_vec();
        let mut _cs = cs.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MCExp(
                self.sid,
                _b.len() as u64,
                _b.as_mut_ptr() as *mut i32,
                ph,
                _cs.len() as u64,
                _cs.as_mut_ptr(),
                _q.as_mut_ptr() as *mut u64,
            );
        }
        self.check_error()
    }

    pub fn swap(&self, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Swap Gate
        //
        // Swaps the qubits at two given positions.
        //
        // Args:
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::SWAP(self.sid, qi1, qi2);
        }
        self.check_error()
    }

    pub fn iswap(&self, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Swap Gate with phase.
        //
        // Swaps the qubits at two given positions.
        // If the bits are different then there is additional phase of `i`.
        //
        // Args:
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::ISWAP(self.sid, qi1, qi2);
        }
        self.check_error()
    }

    pub fn adjiswap(&self, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Swap Gate with phase.
        //
        // Swaps the qubits at two given positions.
        // If the bits are different then there is additional phase of `-i`.
        //
        // Args:
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::AdjISWAP(self.sid, qi1, qi2);
        }
        self.check_error()
    }

    pub fn fsim(&self, th: f64, ph: f64, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Fsim gate.
        //
        // The 2-qubit “fSim” gate
        // Useful in the simulation of particles with fermionic statistics
        //
        // Args:
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::FSim(self.sid, th, ph, qi1, qi2);
        }
        self.check_error()
    }

    pub fn cswap(&self, c: Vec<u64>, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Controlled-swap Gate
        //
        // Swaps the qubits at two given positions if the control qubits are `|1>`
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::CSWAP(self.sid, _c.len() as u64, _c.as_mut_ptr(), qi1, qi2);
        }
        self.check_error()
    }

    pub fn acswap(&self, c: Vec<u64>, qi1: u64, qi2: u64) -> Result<(), QrackError> {
        // Anti controlled-swap Gate
        //
        // Swaps the qubits at two given positions if the control qubits are `|0>`
        //
        // Args:
        //     c(Vec<u64>): list of controlled qubits.
        //     qi1(u64): First position of qubit.
        //     qi2(u64): Second position of qubit.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _c = c.to_vec();
        unsafe {
            qrack_system::ACSWAP(self.sid, _c.len() as u64, _c.as_mut_ptr(), qi1, qi2);
        }
        self.check_error()
    }

    // standard operations
    pub fn m(&self, q: u64) -> Result<u64, QrackError> {
        // Measurement gate
        //
        // Measures the qubit at "q" and returns Boolean value.
        // This operator is not unitary & is probabilistic in nature.
        //
        // Args:
        //     q(u64): qubit to measure
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Measurement result.

        let result: u64;
        unsafe {
            result = qrack_system::M(self.sid, q);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn force_m(&self, q: u64, r: bool) -> Result<u64, QrackError> {
        // Force-Measurement gate
        //
        // Acts as if the measurement is applied and the result obtained is `r`
        //
        // Args:
        //     q(u64): qubit to measure
        //     r(bool): the required result
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Measurement result.

        let result: u64;
        unsafe {
            result = qrack_system::ForceM(self.sid, q, r);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn m_all(&self) -> Result<u64, QrackError> {
        // Measure-all gate
        //
        // Measures measures all qubits.
        // This operator is not unitary & is probabilistic in nature.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Measurement result of all qubits.

        let result: u64;
        unsafe {
            result = qrack_system::MAll(self.sid);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn measure_pauli(&self, b: Vec<Pauli>, q: Vec<u64>) -> Result<u64, QrackError> {
        // Pauli Measurement gate
        //
        // Measures the qubits at "q" with the given pauli bases.
        // This operator is not unitary & is probabilistic in nature.
        //
        // Args:
        //     b(Pauli): Pauli basis
        //     q(u64): qubit to measure
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Measurement result.

        if b.len() != q.len() {
            return Err(QrackError {});
        }
        let mut _b = b.to_vec();
        let mut _q = q.to_vec();
        let result: u64;
        unsafe {
            result = qrack_system::Measure(
                self.sid,
                _b.len() as u64,
                _b.as_mut_ptr() as *mut i32,
                _q.as_mut_ptr(),
            );
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn measure_shots(&self, q: Vec<u64>, s: u64) -> Result<Vec<u64>, QrackError> {
        // Multi-shot measurement operator
        //
        // Samples the qubits at "q" with the given pauli bases.
        // This operator is probabilistic in nature.
        //
        // Args:
        //     q(Vec<u64>): list of qubits to measure
        //     s(u64): number of shots
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Vec<u64> of measurement result.

        let mut _q = q.to_vec();
        let mut result = vec![0; s as usize];
        unsafe {
            qrack_system::MeasureShots(
                self.sid,
                _q.len() as u64,
                _q.as_mut_ptr(),
                s,
                result.as_mut_ptr(),
            );
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn reset_all(&self) -> Result<(), QrackError> {
        // Reset gate
        //
        // Resets all qubits to `|0>`
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::ResetAll(self.sid);
        }
        self.check_error()
    }

    // arithmetic-logic-unit (ALU)
    pub fn add(&self, a: Vec<u64>, q: Vec<u64>) -> Result<(), QrackError> {
        // Add integer to qubit
        //
        // Adds the given integer to the given set of qubits.
        //
        // Args:
        //     a(Vec<u64>): number to add (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::ADD(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn sub(&self, a: Vec<u64>, q: Vec<u64>) -> Result<(), QrackError> {
        // Subtract integer to qubit
        //
        // Subtracts the given integer to the given set of qubits.
        //
        // Args:
        //     a(Vec<u64>): number to add (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::SUB(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn adds(&self, a: Vec<u64>, s: u64, q: Vec<u64>) -> Result<(), QrackError> {
        // Signed Addition integer to qubit
        //
        // Signed Addition of the given integer to the given set of qubits,
        // if there is an overflow the resultant will become negative.
        //
        // Args:
        //     a(Vec<u64>): number to add (as u64 words, low-to-high)
        //     s: qubit to store overflow
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::ADDS(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                s,
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn subs(&self, a: Vec<u64>, s: u64, q: Vec<u64>) -> Result<(), QrackError> {
        // Signed Subtraction integer to qubit
        //
        // Signed Subtraction of the given integer to the given set of qubits,
        // if there is an overflow the resultant will become negative.
        //
        // Args:
        //     a(Vec<u64>): number to subtract (as u64 words, low-to-high)
        //     s: qubit to store overflow
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::SUBS(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                s,
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mul(&self, a: Vec<u64>, q: Vec<u64>, o: Vec<u64>) -> Result<(), QrackError> {
        // Multiplies integer to qubits
        //
        // Multiplies the given integer to the given set of qubits.
        // Carry register is required for maintaining the unitary nature of
        // operation and must be as long as the input qubit register.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): carry register
        //
        // Raises:
        //    RuntimeError: QrackSimulator raised an exception.

        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MUL(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn div(&self, a: Vec<u64>, q: Vec<u64>, o: Vec<u64>) -> Result<(), QrackError> {
        // Divides qubit by integer
        //
        // 'Divides' the given qubits by the integer.
        // (Specifically, this is rather the adjoint of "mul().")
        // Carry register is required for maintaining the unitary nature of
        // operation.
        //
        // Args:
        //     a(Vec<u64>): number by which to divide (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to divide
        //     o(Vec<u64>): carry register
        //
        // Raises:
        //    RuntimeError: QrackSimulator raised an exception.

        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::DIV(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn muln(
        &self,
        a: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Modulo Multiplication
        //
        // Modulo Multiplication of the given integer to the given set of qubits
        // Out-of-place register is required to store the resultant.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MULN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn divn(
        &self,
        a: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Modulo Division
        //
        // 'Modulo Division' of the given set of qubits by the given integer
        // (Specifically, this is rather the adjoint of "muln().")
        // Out-of-place register is required to store the resultant.
        //
        // Args:
        //     a(Vec<u64>): number by which to divide (as u64 words, low-to-high)
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to divide
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::DIVN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn pown(
        &self,
        a: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Modulo Power
        //
        // Raises the qubit to the power `a` to which `mod m` is applied to.
        // Out-of-place register is required to store the resultant.
        //
        // Args:
        //     a(Vec<u64>): power by which to raise (as u64 words, low-to-high)
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to exponentiate
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::POWN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcadd(&self, a: Vec<u64>, c: Vec<u64>, q: Vec<u64>) -> Result<(), QrackError> {
        // Controlled-add
        //
        // Adds the given integer to the given set of qubits if all controlled
        // qubits are `|1>`.
        //
        // Args:
        //     a(Vec<u64>): number to add (as u64 words, low-to-high)
        //     c(Vec<u64>): list of controlled qubits.
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MCADD(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcsub(&self, a: Vec<u64>, c: Vec<u64>, q: Vec<u64>) -> Result<(), QrackError> {
        // Controlled-subtract
        //
        // Subtracts the given integer from the given set of qubits if all controlled
        // qubits are `|1>`.
        //
        // Args:
        //     a(Vec<u64>): number to subtract (as u64 words, low-to-high)
        //     c(Vec<u64>): list of controlled qubits.
        //     q(Vec<u64>): list of qubits to add the number
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _q = q.to_vec();
        unsafe {
            qrack_system::MCSUB(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcmul(
        &self,
        a: Vec<u64>,
        c: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Controlled-multiply
        //
        // Multiplies the given integer to the given set of qubits if all controlled
        // qubits are `|1>`.
        // Carry register is required for maintaining the unitary nature of
        // operation and must be as long as the input qubit register.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     c(Vec<u64>): list of control qubits
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): carry register
        //
        // Raises:
        //    RuntimeError: QrackSimulator raised an exception.

        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MCMUL(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcdiv(
        &self,
        a: Vec<u64>,
        c: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Controlled-divide.
        //
        // 'Divides' the given qubits by the integer if all controlled
        // qubits are `|1>`.
        // Carry register is required for maintaining the unitary nature of
        // operation.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     c(Vec<u64>): list of control qubits
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): carry register
        //
        // Raises:
        //    RuntimeError: QrackSimulator raised an exception.

        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MCDIV(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcmuln(
        &self,
        a: Vec<u64>,
        c: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Controlled-modulo multiplication
        //
        // Modulo Multiplication of the given integer to the given set of qubits
        // if all controlled qubits are `|1>`
        // Out-of-place register is required to store the resultant.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     c(Vec<u64>): list of control qubits
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MCMULN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcdivn(
        &self,
        a: Vec<u64>,
        c: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Controlled-modulo multiplication
        //
        // Modulo division of the given integer to the given set of qubits
        // if all controlled qubits are `|1>`
        // Carry register is required for maintaining the unitary nature of
        // operation.
        //
        // Args:
        //     a(Vec<u64>): number by which to divide (as u64 words, low-to-high)
        //     c(Vec<u64>): list of control qubits
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to divide
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MCDIVN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn mcpown(
        &self,
        a: Vec<u64>,
        c: Vec<u64>,
        m: Vec<u64>,
        q: Vec<u64>,
        o: Vec<u64>,
    ) -> Result<(), QrackError> {
        // Controlled-modulo Power
        //
        // Raises the qubit to the power `a` to which `mod m` is applied to if
        // all the controlled qubits are set to `|1>`.
        // Out-of-place register is required to store the resultant.
        //
        // Args:
        //     a(Vec<u64>): number to multiply (as u64 words, low-to-high)
        //     c(Vec<u64>): list of control qubits
        //     m(Vec<u64>): modulo number (as u64 words, low-to-high)
        //     q(Vec<u64>): list of qubits to multiply the number
        //     o(Vec<u64>): result register
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if a.len() != m.len() {
            return Err(QrackError {});
        }
        if q.len() != o.len() {
            return Err(QrackError {});
        }
        let mut _a = a.to_vec();
        let mut _c = c.to_vec();
        let mut _m = m.to_vec();
        let mut _q = q.to_vec();
        let mut _o = o.to_vec();
        unsafe {
            qrack_system::MCPOWN(
                self.sid,
                _a.len() as u64,
                _a.as_mut_ptr(),
                _c.len() as u64,
                _c.as_mut_ptr(),
                _m.as_mut_ptr(),
                _q.len() as u64,
                _q.as_mut_ptr(),
                _o.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn lda(&self, qi: Vec<u64>, qv: Vec<u64>, t: Vec<u8>) -> Result<(), QrackError> {
        // Load Accumalator
        //
        // Quantum counterpart for LDA from MOS-6502 assembly. `t` must be of
        // the length `(1 << qi.len()) * qv.len() / 8`. It loads each list entry index of t into
        // the qi register and each list entry value into the qv register.
        //
        // Args:
        //     qi(Vec<u64>): qubit register for index
        //     qv(Vec<u64>): qubit register for value
        //     t(Vec<u8>): list of values (in minimum u8 footprint per value)
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if (8 * t.len()) < ((1 << qi.len()) * qv.len()) {
            return Err(QrackError {});
        }
        let mut _qi = qi.to_vec();
        let mut _qv = qv.to_vec();
        let mut _t = t.to_vec();
        unsafe {
            qrack_system::LDA(
                self.sid,
                _qi.len() as u64,
                _qi.as_mut_ptr(),
                _qv.len() as u64,
                _qv.as_mut_ptr(),
                _t.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn adc(&self, s: u64, qi: Vec<u64>, qv: Vec<u64>, t: Vec<u8>) -> Result<(), QrackError> {
        // Add with Carry
        //
        // Quantum counterpart for ADC from MOS-6502 assembly. `t` must be of
        // the length `(1 << qi.len()) * qv.len() / 8`.
        // Args:
        //     s(u64): carry qubit index
        //     qi(Vec<u64>): qubit register for index
        //     qv(Vec<u64>): qubit register for value
        //     t(Vec<u8>): list of values (in minimum u8 footprint per value)
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if (8 * t.len()) < ((1 << qi.len()) * qv.len()) {
            return Err(QrackError {});
        }
        let mut _qi = qi.to_vec();
        let mut _qv = qv.to_vec();
        let mut _t = t.to_vec();
        unsafe {
            qrack_system::ADC(
                self.sid,
                s,
                _qi.len() as u64,
                _qi.as_mut_ptr(),
                _qv.len() as u64,
                _qv.as_mut_ptr(),
                _t.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn sbc(&self, s: u64, qi: Vec<u64>, qv: Vec<u64>, t: Vec<u8>) -> Result<(), QrackError> {
        // Subtract with Carry
        //
        // Quantum counterpart for SBC from MOS-6502 assembly. `t` must be of
        // the length `(1 << qi.len()) * qv.len() / 8`.
        // Args:
        //     s(u64): carry qubit index
        //     qi(Vec<u64>): qubit register for index
        //     qv(Vec<u64>): qubit register for value
        //     t(Vec<u8>): list of values (in minimum u8 footprint per value)
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if (8 * t.len()) < ((1 << qi.len()) * qv.len()) {
            return Err(QrackError {});
        }
        let mut _qi = qi.to_vec();
        let mut _qv = qv.to_vec();
        let mut _t = t.to_vec();
        unsafe {
            qrack_system::SBC(
                self.sid,
                s,
                _qi.len() as u64,
                _qi.as_mut_ptr(),
                _qv.len() as u64,
                _qv.as_mut_ptr(),
                _t.as_mut_ptr(),
            );
        }
        self.check_error()
    }

    pub fn hash(&self, q: Vec<u64>, t: Vec<u8>) -> Result<(), QrackError> {
        // Hash function
        //
        // Replicates the behaviour of LDA without the index register.
        // For the operation to be unitary, the entries present in `t` must be
        // unique, and the length of `(1 << q.len()) / 8`.
        //
        // Args:
        //     q(Vec<u64>): qubit register for value
        //     t(Vec<u8>): list of values (in minimum u8 footprint per value)
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        if (8 * t.len()) < (1 << q.len()) {
            return Err(QrackError {});
        }
        let mut _q = q.to_vec();
        let mut _t = t.to_vec();
        unsafe {
            qrack_system::Hash(self.sid, _q.len() as u64, _q.as_mut_ptr(), _t.as_mut_ptr());
        }
        self.check_error()
    }

    // boolean logic gates
    pub fn qand(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical AND
        //
        // Logical AND of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::AND(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn qor(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical OR
        //
        // Logical OR of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::OR(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn qxor(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical XOR
        //
        // Logical XOR of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::XOR(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn qnand(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical NAND
        //
        // Logical NAND of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::NAND(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn qnor(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical NOR
        //
        // Logical NOR of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::NOR(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn qxnor(&self, qi1: u64, qi2: u64, qo: u64) -> Result<(), QrackError> {
        // Logical XNOR
        //
        // Logical XNOR of 2 qubits whose result is stored in the target qubit.
        //
        // Args:
        //     qi1(u64): qubit 1
        //     qi2(u64): qubit 2
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::XNOR(self.sid, qi1, qi2, qo);
        }
        self.check_error()
    }

    pub fn cland(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical AND
        //
        // Logical AND with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLAND(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    pub fn clor(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical OR
        //
        // Logical OR with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLOR(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    pub fn clxor(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical XOR
        //
        // Logical XOR with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLXOR(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    pub fn clnand(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical NAND
        //
        // Logical NAND with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLNAND(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    pub fn clnor(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical NOR
        //
        // Logical NOR with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLNOR(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    pub fn clxnor(&self, ci: bool, qi: u64, qo: u64) -> Result<(), QrackError> {
        // Classical XNOR
        //
        // Logical XNOR with one qubit and one classical bit whose result is
        // stored in target qubit.
        //
        // Args:
        //     ci(bool): classical bit
        //     qi(u64): qubit
        //     qo(u64): target qubit
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::CLXNOR(self.sid, ci, qi, qo);
        }
        self.check_error()
    }

    // Particular Quantum Circuits

    // fourier transform
    pub fn qft(&self, qs: Vec<u64>) -> Result<(), QrackError> {
        // Quantum Fourier Transform
        //
        // Applies Quantum Fourier Transform on the list of qubits provided.
        //
        // Args:
        //     qs(Vec<u64>): list of qubits
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _qs = qs.to_vec();
        unsafe {
            qrack_system::QFT(self.sid, _qs.len() as u64, _qs.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn iqft(&self, qs: Vec<u64>) -> Result<(), QrackError> {
        // Inverse-quantum Fourier Transform
        //
        // Applies Inverse-quantum Fourier Transform on the list of qubits
        // provided.
        //
        // Args:
        //     qs(Vec<u64>): list of qubits
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _qs = qs.to_vec();
        unsafe {
            qrack_system::IQFT(self.sid, _qs.len() as u64, _qs.as_mut_ptr());
        }
        self.check_error()
    }

    // pseudo-quantum

    // allocate and release
    pub fn allocate_qubit(&self, qid: u64) -> Result<(), QrackError> {
        // Allocate Qubit
        //
        // Allocate 1 new qubit with the given qubit ID.
        //
        // Args:
        //    qid(u64): qubit id
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::allocateQubit(self.sid, qid);
        }
        self.check_error()
    }

    pub fn release(&self, q: u64) -> Result<bool, QrackError> {
        // Release Qubit
        //
        // Release qubit given by the given qubit ID.
        //
        // Args:
        //     q(u64): qubit id
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     If the qubit was in `|0>` state with small tolerance.

        let result: bool;
        unsafe {
            result = qrack_system::release(self.sid, q);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn num_qubits(&self) -> Result<u64, QrackError> {
        // Get Qubit count
        //
        // Returns the qubit count of the simulator.
        //
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Qubit count of the simulator

        let result: u64;
        unsafe {
            result = qrack_system::num_qubits(self.sid);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    // schmidt decomposition
    pub fn compose(&self, other: QrackSimulator, q: Vec<u64>) -> Result<(), QrackError> {
        // Compose qubits
        //
        // Compose quantum description of given qubit with the current system.
        //
        // Args:
        //    other(QrackSimulator): other QrackSimulator to insert
        //    q(Vec<u64>): qubit ids in 'other' to compose
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::Compose(self.sid, other.sid, _q.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn decompose(&self, q: Vec<u64>) -> Result<QrackSimulator, QrackError> {
        // Decompose system
        //
        // Decompose the given qubit out of the system.
        // Warning: The qubit subsystem state must be separable, or the behavior
        // of this method is undefined.
        //
        // Args:
        //     q(Vec<u64>): qubit ids of subsystem to decompose
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //    new QrackSimulator with decomposed subsytem

        let mut other = QrackSimulator::new(0).unwrap();
        unsafe {
            qrack_system::destroy(other.sid);
        }
        let mut _q = q.to_vec();
        unsafe {
            other.sid = qrack_system::Decompose(self.sid, _q.len() as u64, _q.as_mut_ptr());
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(other)
    }

    pub fn dispose(&self, q: Vec<u64>) -> Result<(), QrackError> {
        // Dispose qubits
        //
        // Minimally decompose a set of contiguous bits from the separably
        // composed unit, and discard the separable bits.
        // Warning: The qubit subsystem state must be separable, or the behavior
        // of this method is undefined.
        //
        // Args:
        //     q(Vec<u64>): qubit ids of subsystem to dispose
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::Dispose(self.sid, _q.len() as u64, _q.as_mut_ptr());
        }
        self.check_error()
    }

    // pub fn in_ket(&self, ket: *mut f32) -> Result<(), QrackError> {
    // Set state vector
    //
    // Set state vector for the selected simulator ID.
    //
    // Warning: State vector is not always the internal representation, leading
    // to sub-optimal performance of the method.
    //
    // Note: By design of this interface layer between Rust and C++, Qrack
    // cannot guarantee that your input ket is not insecurely mutated, but it
    // "promises" you that it does not intend to ever mutate it.
    //
    // Args:
    //     ket(*mut f32): the state vector to which simulator will be set
    //
    // Raises:
    //     RuntimeError: Not implemented for the given builds.

    //     unsafe {
    //         qrack_system::InKet(self.sid, ket);
    //     }
    //     self.check_error()
    // }

    // pub fn out_ket(self, ket: *mut f32) -> Result<(), QrackError> {
    // Get state vector
    //
    // Returns the raw state vector of the simulator.
    //
    // Warning: State vector is not always the internal representation, leading
    // to sub-optimal performance of the method.
    //
    // Args:
    //     ket(*mut f32): the state vector to which simulator will be output
    //
    // Raises:
    //     RuntimeError: Not implemented for the given builds.

    //     unsafe {
    //         qrack_system::OutKet(self.sid, ket);
    //     }
    //     self.check_error()
    // }

    pub fn prob_perm(&self, q: Vec<u64>, c: Vec<bool>) -> Result<f64, QrackError> {
        // Probability of permutation
        //
        // Get the probability that the qubit IDs in "q" have the truth values
        // in "c", directly corresponding by vector index.
        //
        // Args:
        //    q(Vec<u64>): qubit ids
        //    c(Vec<bool>): qubit truth values
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     probability that each qubit in "q[i]" has corresponding truth
        //     value in "c[i]", at once

        if q.len() != c.len() {
            return Err(QrackError {});
        }
        let mut _q = q.to_vec();
        let mut _c = c.to_vec();
        let result: f64;
        unsafe {
            result = qrack_system::PermutationProb(
                self.sid,
                _q.len() as u64,
                _q.as_mut_ptr(),
                _c.as_mut_ptr(),
            );
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn prob(&self, q: u64) -> Result<f64, QrackError> {
        // Probability of `|1>`
        //
        // Get the probability that a qubit is in the `|1>` state.
        //
        // Args:
        //     q(u64): qubit id
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     probability of qubit being in `|1>`

        let result: f64;
        unsafe {
            result = qrack_system::Prob(self.sid, q);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn permutation_expectation(&self, c: Vec<u64>) -> Result<f64, QrackError> {
        // Permutation expectation value
        //
        // Get the permutation expectation value, based upon the order of
        // input qubits.
        //
        // Args:
        //     c(Vec<u64>): permutation (as u64 words, low-to-high)
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Expectation value

        let mut _c = c.to_vec();
        let result: f64;
        unsafe {
            result =
                qrack_system::PermutationExpectation(self.sid, _c.len() as u64, _c.as_mut_ptr());
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn joint_ensemble_probability(
        &self,
        b: Vec<Pauli>,
        q: Vec<u64>,
    ) -> Result<f64, QrackError> {
        // Ensemble probability
        //
        // Find the joint probability for all specified qubits under the
        // respective Pauli basis transformations.
        //
        // Args:
        //     b(Pauli): pauli basis
        //     q(Vec<u64>): specified qubits
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Expectation value

        if b.len() != q.len() {
            return Err(QrackError {});
        }
        let mut _b = b.to_vec();
        let mut _q = q.to_vec();
        let result: f64;
        unsafe {
            result = qrack_system::JointEnsembleProbability(
                self.sid,
                _b.len() as u64,
                _b.as_mut_ptr() as *mut i32,
                _q.as_mut_ptr(),
            );
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn phase_parity(&self, la: f64, q: Vec<u64>) -> Result<(), QrackError> {
        // Phase to odd parity
        //
        // Applies `e^(i*la)` phase factor to all combinations of bits with
        // odd parity, based upon permutations of qubits.
        //
        // Args:
        //     la(f64): phase
        //     q(Vec<u64>): specified qubits
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        let mut _q = q.to_vec();
        unsafe {
            qrack_system::PhaseParity(self.sid, la, _q.len() as u64, _q.as_mut_ptr());
        }
        self.check_error()
    }

    pub fn try_separate_1qb(&self, qi1: u64) -> Result<bool, QrackError> {
        // Manual seperation
        //
        // Exposes manual control for schmidt decomposition which attempts to
        // decompose the qubit with possible performance improvement
        //
        // Args:
        //     qi1(u64): qubit to be decomposed
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Success/failure of separation attempt

        let result: bool;
        unsafe {
            result = qrack_system::TrySeparate1Qb(self.sid, qi1);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn try_separate_2qb(&self, qi1: u64, qi2: u64) -> Result<bool, QrackError> {
        // Manual two-qubits seperation
        //
        // two-qubits counterpart of `try_separate_1qb`.
        //
        // Args:
        //     qi1(u64): qubit 1 in subsystem to be decomposed
        //     qi2(u64): qubit 2 in subsystem to be decomposed
        //
        // Raises:
        //     Runtimeerror: QrackSimulator raised an exception.
        //
        // Returns:
        //     Success/failure of separation attempt

        let result: bool;
        unsafe {
            result = qrack_system::TrySeparate2Qb(self.sid, qi1, qi2);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn try_separate_tolerance(&self, qs: Vec<u64>, t: f64) -> Result<bool, QrackError> {
        // Manual multi-qubits seperation
        //
        // Multi-qubits counterpart of `try_separate_1qb`.
        //
        // Args:
        //     qs: list of qubits to be decomposed
        //     t: allowed Schmidt decomposition rounding parameter (SDRP) tolerance
        //
        // Raises:
        //     Runtimeerror: QrackSimulator raised an exception.
        //
        // Returns:
        //     Success/failure of separation attempt

        let mut _qs = qs.to_vec();
        let result: bool;
        unsafe {
            result = qrack_system::TrySeparateTol(self.sid, _qs.len() as u64, _qs.as_mut_ptr(), t);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn get_unitary_fidelity(&self) -> Result<f64, QrackError> {
        // Get fidelity estimate
        //
        // When using "Schmidt decomposition rounding parameter" ("SDRP")
        // approximate simulation, QrackSimulator() can make an excellent
        // estimate of its overall fidelity at any time, tested against a
        // nearest-neighbor variant of quantum volume circuits.
        //
        // Resetting the fidelity calculation to 1.0 happens automatically
        // when calling `mall` are can be done manually with
        // `reset_unitary_fidelity()`.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        //
        // Returns:
        //     Fidelity estimate
        let result: f64;
        unsafe {
            result = qrack_system::GetUnitaryFidelity(self.sid);
        }
        if self.get_error() != 0 {
            return Err(QrackError {});
        }
        Ok(result)
    }

    pub fn reset_unitary_fidelity(&self) -> Result<(), QrackError> {
        // Reset fidelity estimate
        //
        // When using "Schmidt decomposition rounding parameter" ("SDRP")
        // approximate simulation, QrackSimulator() can make an excellent
        // estimate of its overall fidelity at any time, tested against a
        // nearest-neighbor variant of quantum volume circuits.
        //
        // Resetting the fidelity calculation to 1.0 happens automatically
        // when calling `m_all` are can be done manually with
        // `reset_unitary_fidelity()`.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        unsafe {
            qrack_system::ResetUnitaryFidelity(self.sid);
        }
        self.check_error()
    }

    pub fn set_sdrp(&self, sdrp: f64) -> Result<(), QrackError> {
        // Reset fidelity estimate
        //
        // When using "Schmidt decomposition rounding parameter" ("SDRP")
        // approximate simulation, QrackSimulator() can make an excellent
        // estimate of its overall fidelity at any time, tested against a
        // nearest-neighbor variant of quantum volume circuits.
        //
        // Resetting the fidelity calculation to 1.0 happens automatically
        // when calling `m_all` are can be done manually with
        // `reset_unitary_fidelity()`.
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.
        unsafe {
            qrack_system::SetSdrp(self.sid, sdrp);
        }
        self.check_error()
    }

    pub fn set_reactive_separate(&self, irs: bool) -> Result<(), QrackError> {
        // Set reactive separation option
        //
        // If reactive separation is available, then this method turns it off/on.
        // Note that reactive separation is on by default.
        //
        // Args:
        //     irs(bool): on/off for aggressive separation
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::SetReactiveSeparate(self.sid, irs);
        }
        self.check_error()
    }

    pub fn set_t_injection(self, iti: bool) -> Result<(), QrackError> {
        // Set t-injection option
        //
        // If t-injection is available, then this method turns it off/on.
        // Note that t-injection is on by default.
        //
        // Args:
        //     iti(bool): on/off for "reverse t-injection gadget"
        //
        // Raises:
        //     RuntimeError: QrackSimulator raised an exception.

        unsafe {
            qrack_system::SetTInjection(self.sid, iti);
        }
        self.check_error()
    }
}
