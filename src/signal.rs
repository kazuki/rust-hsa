use std::mem::zeroed;
use std::collections::HashMap;
use super::{check, ErrorStatus};
use native::*;

pub struct Signal {
    handle: SignalHandle,
}

pub struct SignalGroup<T> {
    handle: SignalGroupHandle,
    signals: HashMap<u64, T>,
}

pub trait SignalBase {
    fn handle(&self) -> SignalHandle;

    fn load_scacquire(&self) -> SignalValue {
        unsafe { hsa_signal_load_scacquire(self.handle()) }
    }

    fn load_relaxed(&self) -> SignalValue {
        unsafe { hsa_signal_load_relaxed(self.handle()) }
    }

    #[deprecated]
    fn load_acquire(&self) -> SignalValue {
        unsafe { hsa_signal_load_acquire(self.handle()) }
    }

    fn store_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_store_relaxed(self.handle(), value) }
    }

    fn store_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_store_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn store_release(&self, value: SignalValue) {
        unsafe { hsa_signal_store_release(self.handle(), value) }
    }

    fn silent_store_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_silent_store_relaxed(self.handle(), value) }
    }

    fn silent_store_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_silent_store_screlease(self.handle(), value) }
    }

    fn exchange_scacq_screl(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_scacq_screl(self.handle(), value) }
    }

    fn exchange_scacquire(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_scacquire(self.handle(), value) }
    }

    fn exchange_relaxed(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_relaxed(self.handle(), value) }
    }

    fn exchange_screlease(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn exchange_acq_rel(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn exchange_acquire(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn exchange_release(&self, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_exchange_release(self.handle(), value) }
    }

    fn cas_scacq_screl(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_scacq_screl(self.handle(), expected, value) }
    }

    fn cas_scacquire(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_scacquire(self.handle(), expected, value) }
    }

    fn cas_relaxed(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_relaxed(self.handle(), expected, value) }
    }

    fn cas_screlease(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_screlease(self.handle(), expected, value) }
    }

    #[deprecated]
    fn cas_acq_rel(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_acq_rel(self.handle(), expected, value) }
    }

    #[deprecated]
    fn cas_acquire(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_acquire(self.handle(), expected, value) }
    }

    #[deprecated]
    fn cas_release(&self, expected: SignalValue, value: SignalValue) -> SignalValue {
        unsafe { hsa_signal_cas_release(self.handle(), expected, value) }
    }

    fn add_scacq_screl(&self, value: SignalValue) {
        unsafe { hsa_signal_add_scacq_screl(self.handle(), value) }
    }

    fn add_scacquire(&self, value: SignalValue) {
        unsafe { hsa_signal_add_scacquire(self.handle(), value) }
    }

    fn add_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_add_relaxed(self.handle(), value) }
    }

    fn add_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_add_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn add_acq_rel(&self, value: SignalValue) {
        unsafe { hsa_signal_add_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn add_acquire(&self, value: SignalValue) {
        unsafe { hsa_signal_add_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn add_release(&self, value: SignalValue) {
        unsafe { hsa_signal_add_release(self.handle(), value) }
    }

    fn subtract_scacq_screl(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_scacq_screl(self.handle(), value) }
    }

    fn subtract_scacquire(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_scacquire(self.handle(), value) }
    }

    fn subtract_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_relaxed(self.handle(), value) }
    }

    fn subtract_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn subtract_acq_rel(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn subtract_acquire(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn subtract_release(&self, value: SignalValue) {
        unsafe { hsa_signal_subtract_release(self.handle(), value) }
    }

    fn and_scacq_screl(&self, value: SignalValue) {
        unsafe { hsa_signal_and_scacq_screl(self.handle(), value) }
    }

    fn and_scacquire(&self, value: SignalValue) {
        unsafe { hsa_signal_and_scacquire(self.handle(), value) }
    }

    fn and_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_and_relaxed(self.handle(), value) }
    }

    fn and_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_and_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn and_acq_rel(&self, value: SignalValue) {
        unsafe { hsa_signal_and_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn and_acquire(&self, value: SignalValue) {
        unsafe { hsa_signal_and_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn and_release(&self, value: SignalValue) {
        unsafe { hsa_signal_and_release(self.handle(), value) }
    }

    fn or_scacq_screl(&self, value: SignalValue) {
        unsafe { hsa_signal_or_scacq_screl(self.handle(), value) }
    }

    fn or_scacquire(&self, value: SignalValue) {
        unsafe { hsa_signal_or_scacquire(self.handle(), value) }
    }

    fn or_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_or_relaxed(self.handle(), value) }
    }

    fn or_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_or_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn or_acq_rel(&self, value: SignalValue) {
        unsafe { hsa_signal_or_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn or_acquire(&self, value: SignalValue) {
        unsafe { hsa_signal_or_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn or_release(&self, value: SignalValue) {
        unsafe { hsa_signal_or_release(self.handle(), value) }
    }

    fn xor_scacq_screl(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_scacq_screl(self.handle(), value) }
    }

    fn xor_scacquire(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_scacquire(self.handle(), value) }
    }

    fn xor_relaxed(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_relaxed(self.handle(), value) }
    }

    fn xor_screlease(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_screlease(self.handle(), value) }
    }

    #[deprecated]
    fn xor_acq_rel(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_acq_rel(self.handle(), value) }
    }

    #[deprecated]
    fn xor_acquire(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_acquire(self.handle(), value) }
    }

    #[deprecated]
    fn xor_release(&self, value: SignalValue) {
        unsafe { hsa_signal_xor_release(self.handle(), value) }
    }

    fn wait_scacquire(
        &self,
        condition: SignalCondition,
        compare_value: SignalValue,
        timeout_hint: u64,
        wait_state_hint: WaitState,
    ) -> SignalValue {
        unsafe {
            hsa_signal_wait_scacquire(
                self.handle(),
                condition,
                compare_value,
                timeout_hint,
                wait_state_hint,
            )
        }
    }

    fn wait_relaxed(
        &self,
        condition: SignalCondition,
        compare_value: SignalValue,
        timeout_hint: u64,
        wait_state_hint: WaitState,
    ) -> SignalValue {
        unsafe {
            hsa_signal_wait_relaxed(
                self.handle(),
                condition,
                compare_value,
                timeout_hint,
                wait_state_hint,
            )
        }
    }

    #[deprecated]
    fn wait_acquire(
        &self,
        condition: SignalCondition,
        compare_value: SignalValue,
        timeout_hint: u64,
        wait_state_hint: WaitState,
    ) -> SignalValue {
        unsafe {
            hsa_signal_wait_acquire(
                self.handle(),
                condition,
                compare_value,
                timeout_hint,
                wait_state_hint,
            )
        }
    }
}

impl SignalBase for SignalHandle {
    fn handle(&self) -> SignalHandle {
        *self
    }
}

impl SignalBase for Signal {
    fn handle(&self) -> SignalHandle {
        self.handle
    }
}

impl Signal {
    pub fn new(initial_value: SignalValue, consumers: &[Agent]) -> Result<Signal, ErrorStatus> {
        let v = {
            let mut tmp = Vec::new();
            tmp.extend_from_slice(consumers);
            tmp
        };
        unsafe {
            let handle: SignalHandle = {
                zeroed()
            };
            check(
                hsa_signal_create(initial_value, v.len() as u32, v.as_ptr(), &handle),
                (),
            ).map(|_| Signal { handle: handle })
        }
    }
}

impl Drop for Signal {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_signal_destroy(self.handle);
            }
            self.handle.handle = 0;
        }
    }
}

impl<T: Clone + AsRef<Signal>> SignalGroup<T> {
    pub fn new(signals: &[T], consumers: &[Agent]) -> Result<SignalGroup<T>, ErrorStatus> {
        let (signals, signals_map) = {
            let mut t0 = Vec::new();
            let mut t1 = HashMap::new();
            for s in signals {
                let h = s.as_ref().handle;
                t0.push(h);
                t1.insert(h.handle, s.clone());
            }
            (t0, t1)
        };
        let consumers = {
            let mut tmp = Vec::new();
            tmp.extend_from_slice(consumers);
            tmp
        };
        unsafe {
            let handle: SignalGroupHandle = zeroed();
            check(
                hsa_signal_group_create(
                    signals.len() as u32,
                    signals.as_ptr(),
                    consumers.len() as u32,
                    consumers.as_ptr(),
                    &handle,
                ),
                (),
            ).map(|_| {
                SignalGroup {
                    handle: handle,
                    signals: signals_map,
                }
            })
        }
    }

    pub fn wait_any_scacquire(
        &self,
        conditions: &[SignalCondition],
        compare_values: &[SignalValue],
        wait_state_hint: WaitState,
    ) -> Result<(T, SignalValue), ErrorStatus> {
        self.wait_any(false, conditions, compare_values, wait_state_hint)
    }

    pub fn wait_any_relaxed(
        &self,
        conditions: &[SignalCondition],
        compare_values: &[SignalValue],
        wait_state_hint: WaitState,
    ) -> Result<(T, SignalValue), ErrorStatus> {
        self.wait_any(true, conditions, compare_values, wait_state_hint)
    }

    fn wait_any(
        &self,
        is_relaxed: bool,
        conditions: &[SignalCondition],
        compare_values: &[SignalValue],
        wait_state_hint: WaitState,
    ) -> Result<(T, SignalValue), ErrorStatus> {
        let (conditions, compare_values) = {
            let mut t0 = Vec::new();
            let mut t1 = Vec::new();
            t0.extend_from_slice(conditions);
            t1.extend_from_slice(compare_values);
            (t0, t1)
        };
        unsafe {
            let signal: SignalHandle = zeroed();
            let value: SignalValue = zeroed();
            let r = if is_relaxed {
                hsa_signal_group_wait_any_relaxed(
                    self.handle,
                    conditions.as_ptr(),
                    compare_values.as_ptr(),
                    wait_state_hint,
                    &signal,
                    &value,
                )
            } else {
                hsa_signal_group_wait_any_scacquire(
                    self.handle,
                    conditions.as_ptr(),
                    compare_values.as_ptr(),
                    wait_state_hint,
                    &signal,
                    &value,
                )
            };
            check(r, ()).and_then(|_| match self.signals.get(&signal.handle) {
                Some(s) => Ok((s.clone(), value)),
                _ => Err(ErrorStatus::Exception),
            })
        }
    }
}

impl<T> Drop for SignalGroup<T> {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_signal_group_destroy(self.handle);
            }
            self.handle.handle = 0;
        }
    }
}
