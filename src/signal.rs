use std;
use std::mem;
use std::ops::Drop;

use super::{ErrorType, to_result};
use native::*;

pub struct Signal {
    handle: SignalHandle,
}

impl Signal {
    pub fn new(initial_value: SignalValue, consumers: &Vec<&Agent>) -> Result<Signal, ErrorType> {

        let mut cloned_consumers: Vec<Agent> = Vec::new();
        for agent in consumers.iter() {
            cloned_consumers.push((*agent).clone());
        }
        let mut consumers_ptr = cloned_consumers.as_ptr();
        if consumers.len() == 0 {
            consumers_ptr = std::ptr::null();
        }

        let handle: SignalHandle = unsafe { mem::zeroed() };
        to_result(unsafe {
            hsa_signal_create(initial_value, cloned_consumers.len() as u32,
                              consumers_ptr, &handle)
        }, Signal {
            handle: handle
        })
    }

    pub fn clone_handle(&self) -> SignalHandle {
        self.handle.clone()
    }

    pub fn wait_acquire(&self, condition: SignalCondition, compare_value: SignalValue,
                        timeout_hint: u64, wait_state_hint: WaitState) -> SignalValue {
        unsafe {
            hsa_signal_wait_acquire(self.handle.clone(), condition, compare_value,
                                    timeout_hint, wait_state_hint)
        }
    }

    pub fn wait_relaxed(&self, condition: SignalCondition, compare_value: SignalValue,
                        timeout_hint: u64, wait_state_hint: WaitState) -> SignalValue {
        unsafe {
            hsa_signal_wait_relaxed(self.handle.clone(), condition, compare_value,
                                    timeout_hint, wait_state_hint)
        }
    }

    pub fn load_acquire(&self) -> SignalValue {
        unsafe { hsa_signal_load_acquire(self.handle.clone()) }
    }

    pub fn load_relaxed(&self) -> SignalValue {
        unsafe { hsa_signal_load_relaxed(self.handle.clone()) }
    }

    pub fn store_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_store_relaxed(self.handle.clone(), value) }
    }

    pub fn store_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_store_release(self.handle.clone(), value) }
    }

    pub fn exchange_acq_rel(&mut self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_acq_rel(self.handle.clone(), value) }
    }

    pub fn exchange_acquire(&mut self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_acquire(self.handle.clone(), value) }
    }

    pub fn exchange_relaxed(&mut self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_relaxed(self.handle.clone(), value) }
    }

    pub fn exchange_release(&mut self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_release(self.handle.clone(), value) }
    }

    pub fn cas_acq_rel(&mut self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_acq_rel(self.handle.clone(), expected, value) }
    }

    pub fn cas_acquire(&mut self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_acquire(self.handle.clone(), expected, value) }
    }

    pub fn cas_relaxed(&mut self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_relaxed(self.handle.clone(), expected, value) }
    }

    pub fn cas_release(&mut self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_release(self.handle.clone(), expected, value) }
    }

    pub fn add_acq_rel(&mut self, value: SignalValue) {
        unsafe { hsa_signal_add_acq_rel(self.handle.clone(), value) }
    }

    pub fn add_acquire(&mut self, value: SignalValue) {
        unsafe { hsa_signal_add_acquire(self.handle.clone(), value) }
    }

    pub fn add_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_add_relaxed(self.handle.clone(), value) }
    }

    pub fn add_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_add_release(self.handle.clone(), value) }
    }

    pub fn subtract_acq_rel(&mut self, value: SignalValue) {
        unsafe { hsa_signal_subtract_acq_rel(self.handle.clone(), value) }
    }

    pub fn subtract_acquire(&mut self, value: SignalValue) {
        unsafe { hsa_signal_subtract_acquire(self.handle.clone(), value) }
    }

    pub fn subtract_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_subtract_relaxed(self.handle.clone(), value) }
    }

    pub fn subtract_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_subtract_release(self.handle.clone(), value) }
    }

    pub fn and_acq_rel(&mut self, value: SignalValue) {
        unsafe { hsa_signal_and_acq_rel(self.handle.clone(), value) }
    }

    pub fn and_acquire(&mut self, value: SignalValue) {
        unsafe { hsa_signal_and_acquire(self.handle.clone(), value) }
    }

    pub fn and_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_and_relaxed(self.handle.clone(), value) }
    }

    pub fn and_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_and_release(self.handle.clone(), value) }
    }

    pub fn or_acq_rel(&mut self, value: SignalValue) {
        unsafe { hsa_signal_or_acq_rel(self.handle.clone(), value) }
    }

    pub fn or_acquire(&mut self, value: SignalValue) {
        unsafe { hsa_signal_or_acquire(self.handle.clone(), value) }
    }

    pub fn or_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_or_relaxed(self.handle.clone(), value) }
    }

    pub fn or_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_or_release(self.handle.clone(), value) }
    }

    pub fn xor_acq_rel(&mut self, value: SignalValue) {
        unsafe { hsa_signal_xor_acq_rel(self.handle.clone(), value) }
    }

    pub fn xor_acquire(&mut self, value: SignalValue) {
        unsafe { hsa_signal_xor_acquire(self.handle.clone(), value) }
    }

    pub fn xor_relaxed(&mut self, value: SignalValue) {
        unsafe { hsa_signal_xor_relaxed(self.handle.clone(), value) }
    }

    pub fn xor_release(&mut self, value: SignalValue) {
        unsafe { hsa_signal_xor_release(self.handle.clone(), value) }
    }
}

impl Drop for Signal {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_signal_destroy(self.handle.clone());
            }
            self.handle.handle = 0;
        }
    }
}
