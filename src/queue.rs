use std;
use std::{mem,u32};
use std::ops::Drop;
use super::{ErrorType, to_error_type};
use native::*;

pub struct Queue {
    handle: *const QueueHandle
}

impl Queue {
    pub fn create(agent: &Agent, size: u32, queue_type: QueueType) -> Result<Queue, ErrorType> {
        let handle: *const QueueHandle = unsafe { mem::zeroed() };
        unsafe {
            match hsa_queue_create(agent.clone(), size, queue_type,
                                   std::ptr::null(), std::ptr::null(),
                                   u32::MAX, u32::MAX, &handle) {
                0 => Ok(Queue {
                    handle: handle
                }),
                e => Err(to_error_type(e))
            }
        }
    }

    pub fn enqueue(&mut self, packet: &KernelDispatchPacket) {
        unsafe {
            let handle = &*(self.handle);
            let index = hsa_queue_add_write_index_relaxed(self.handle, 1);
            let queue_index = (index % (handle.size as u64)) as usize;
            let queue_ptr = (handle.base_address as *mut u8).offset((queue_index * PACKET_SIZE) as isize);
            let queue_body_ptr = queue_ptr.offset(4);
            let packet_ptr = packet as *const _ as *const u8;
            let packet_header = *(packet_ptr as *const u32);
            let packet_body_ptr = packet_ptr.offset(4);
            std::ptr::copy(packet_body_ptr, queue_body_ptr, PACKET_SIZE - 4);
            std::intrinsics::atomic_store_rel(queue_ptr as *mut u32,
                                              packet_header);
            hsa_signal_store_relaxed(handle.doorbell_signal.clone(), index as SignalValue);
        }
    }

    /*
    pub fn load_read_index_acquire(&self) -> u64 {
        unsafe {
            hsa_queue_load_read_index_acquire(self.handle)
        }
    }
    pub fn load_read_index_relaxed(&self) -> u64 {
        unsafe {
            hsa_queue_load_read_index_relaxed(self.handle)
        }
    }
    pub fn load_write_index_acquire(&self) -> u64 {
        unsafe {
            hsa_queue_load_write_index_acquire(self.handle)
        }
    }
    pub fn load_write_index_relaxed(&self) -> u64 {
        unsafe {
            hsa_queue_load_write_index_relaxed(self.handle)
        }
    }
    pub fn store_write_index_relaxed(&mut self, value: u64) {
        unsafe {
            hsa_queue_store_write_index_relaxed(self.handle, value)
        }
    }
    pub fn store_write_index_release(&mut self, value: u64) {
        unsafe {
            hsa_queue_store_write_index_release(self.handle, value)
        }
    }
    pub fn cas_write_index_acq_rel(&mut self, expected: u64, value: u64) -> u64 {
        unsafe {
            hsa_queue_cas_write_index_acq_rel(self.handle, expected, value)
        }
    }
    pub fn cas_write_index_acquire(&mut self, expected: u64, value: u64) -> u64 {
        unsafe {
            hsa_queue_cas_write_index_acquire(self.handle, expected, value)
        }
    }
    pub fn cas_write_index_relaxed(&mut self, expected: u64, value: u64) -> u64 {
        unsafe {
            hsa_queue_cas_write_index_relaxed(self.handle, expected, value)
        }
    }
    pub fn cas_write_index_release(&mut self, expected: u64, value: u64) -> u64 {
        unsafe {
            hsa_queue_cas_write_index_release(self.handle, expected, value)
        }
    }
    pub fn add_write_index_acq_rel(&mut self, value: u64) -> u64 {
        unsafe {
            hsa_queue_add_write_index_acq_rel(self.handle, value)
        }
    }
    pub fn add_write_index_acquire(&mut self, value: u64) -> u64 {
        unsafe {
            hsa_queue_add_write_index_acquire(self.handle, value)
        }
    }
    pub fn add_write_index_relaxed(&mut self, value: u64) -> u64 {
        unsafe {
            hsa_queue_add_write_index_relaxed(self.handle, value)
        }
    }
    pub fn add_write_index_release(&mut self, value: u64) -> u64 {
        unsafe {
            hsa_queue_add_write_index_release(self.handle, value)
        }
    }
    pub fn store_read_index_relaxed(&mut self, value: u64) {
        unsafe {
            hsa_queue_store_read_index_relaxed(self.handle, value)
        }
    }
    pub fn store_read_index_release(&mut self, value: u64) {
        unsafe {
            hsa_queue_store_read_index_release(self.handle, value)
        }
    }*/
}

impl Drop for Queue {
    fn drop(&mut self) {
        if self.handle != std::ptr::null() {
            unsafe {
                hsa_queue_destroy(self.handle);
            }
            self.handle = std::ptr::null();
        }
    }
}
