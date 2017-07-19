use std::u32;
use std::os::raw::c_void;
use std::mem::{size_of, zeroed};
use std::ptr::{copy_nonoverlapping, null};
use std::intrinsics::atomic_store_rel;

use native::*;
use super::{check, ErrorStatus};
use signal::*;

pub struct Queue {
    handle: *const QueueHandle,
}

impl Queue {
    pub fn new(agent: Agent, size: u32, typ: QueueType) -> Result<Queue, ErrorStatus> {
        unsafe {
            let handle: *const QueueHandle = zeroed();
            check(
                hsa_queue_create(
                    agent,
                    size,
                    typ,
                    null(),
                    null(),
                    u32::MAX,
                    u32::MAX,
                    &handle,
                ),
                (),
            ).map(|_| Queue { handle: handle })
        }
    }

    pub fn new_soft(
        region: Region,
        size: u32,
        typ: QueueType,
        features: u32,
        doorbell_signal: SignalHandle,
    ) -> Result<Queue, ErrorStatus> {
        unsafe {
            let handle: *const QueueHandle = zeroed();
            check(
                hsa_soft_queue_create(
                    region,
                    size,
                    typ,
                    features,
                    doorbell_signal,
                    &handle
                ),
                (),
            ).map(|_| Queue { handle: handle })
        }
    }

    pub fn inactivate(&self) -> Result<(), ErrorStatus> {
        unsafe { check(hsa_queue_inactivate(self.handle), ()) }
    }

    pub fn load_read_index_scacquire(&self) -> u64 {
        unsafe { hsa_queue_load_read_index_scacquire(self.handle) }
    }
    pub fn load_read_index_relaxed(&self) -> u64 {
        unsafe { hsa_queue_load_read_index_relaxed(self.handle) }
    }
    #[deprecated]
    pub fn load_read_index_acquire(&self) -> u64 {
        unsafe { hsa_queue_load_read_index_acquire(self.handle) }
    }
    pub fn load_write_index_scacquire(&self) -> u64 {
        unsafe { hsa_queue_load_write_index_scacquire(self.handle) }
    }
    pub fn load_write_index_relaxed(&self) -> u64 {
        unsafe { hsa_queue_load_write_index_relaxed(self.handle) }
    }
    #[deprecated]
    pub fn load_write_index_acquire(&self) -> u64 {
        unsafe { hsa_queue_load_write_index_acquire(self.handle) }
    }
    pub fn store_write_index_relaxed(&self, value: u64) {
        unsafe { hsa_queue_store_write_index_relaxed(self.handle, value) }
    }
    pub fn store_write_index_screlease(&self, value: u64) {
        unsafe { hsa_queue_store_write_index_screlease(self.handle, value) }
    }
    #[deprecated]
    pub fn store_write_index_release(&self, value: u64) {
        unsafe { hsa_queue_store_write_index_release(self.handle, value) }
    }

    pub fn cas_write_index_scacq_screl(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_scacq_screl(self.handle, expected, value) }
    }
    pub fn cas_write_index_scacquire(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_scacquire(self.handle, expected, value) }
    }
    pub fn cas_write_index_relaxed(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_relaxed(self.handle, expected, value) }
    }
    pub fn cas_write_index_screlease(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_screlease(self.handle, expected, value) }
    }
    #[deprecated]
    pub fn cas_write_index_acq_rel(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_acq_rel(self.handle, expected, value) }
    }
    #[deprecated]
    pub fn cas_write_index_acquire(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_acquire(self.handle, expected, value) }
    }
    #[deprecated]
    pub fn cas_write_index_release(&self, expected: u64, value: u64) -> u64 {
        unsafe { hsa_queue_cas_write_index_release(self.handle, expected, value) }
    }

    pub fn add_write_index_scacq_screl(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_scacq_screl(self.handle, value) }
    }
    pub fn add_write_index_scacquire(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_scacquire(self.handle, value) }
    }
    pub fn add_write_index_relaxed(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_relaxed(self.handle, value) }
    }
    pub fn add_write_index_screlease(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_screlease(self.handle, value) }
    }
    #[deprecated]
    pub fn add_write_index_acq_rel(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_acq_rel(self.handle, value) }
    }
    #[deprecated]
    pub fn add_write_index_acquire(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_acquire(self.handle, value) }
    }
    #[deprecated]
    pub fn add_write_index_release(&self, value: u64) -> u64 {
        unsafe { hsa_queue_add_write_index_release(self.handle, value) }
    }

    pub fn store_read_index_relaxed(&self, value: u64) {
        unsafe { hsa_queue_store_read_index_relaxed(self.handle, value) }
    }
    pub fn store_read_index_screlease(&self, value: u64) {
        unsafe { hsa_queue_store_read_index_screlease(self.handle, value) }
    }
    #[deprecated]
    pub fn store_read_index_release(&self, value: u64) {
        unsafe { hsa_queue_store_read_index_release(self.handle, value) }
    }

    pub fn size(&self) -> u32 {
        unsafe { (*self.handle).size }
    }
    pub fn base_address(&self) -> *const c_void {
        unsafe { (*self.handle).base_address }
    }
    pub fn doorbell_signal(&self) -> SignalHandle {
        unsafe { (*self.handle).doorbell_signal }
    }

    pub fn copy_kernel_dispatch_packet(&self, packet: &KernelDispatchPacket, index: u64) {
        let src = packet as *const KernelDispatchPacket;
        let rounded_index = (index & ((self.size() - 1) as u64)) as isize;
        let size = size_of::<KernelDispatchPacket>();
        unsafe {
            let dst = (self.base_address() as *mut KernelDispatchPacket).offset(rounded_index);
            let src_u8 = src as *const u8;
            let dst_u8 = dst as *mut u8;
            let src_header = src as *const u16;
            let dst_header = dst as *mut u16;
            copy_nonoverlapping(src_u8.offset(2), dst_u8.offset(2), size - 2);
            atomic_store_rel(dst_header, *src_header);
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            hsa_queue_destroy(self.handle);
        }
    }
}

impl KernelDispatchPacket {
    pub fn new<T0: Sized, T1: SignalBase>(
        packet_type: PacketType,
        fences: &[(FenceScope, PacketHeader)],
        dims: u8,
        workgroup_size: &[u16],
        grid_size: &[u32],
        private_segment_size: u32,
        group_segment_size: u32,
        kernel_object: u64,
        kernarg_address: *const T0,
        completion_signal: T1,
    ) -> Result<KernelDispatchPacket, ()> {
        if dims == 0 || dims as usize != workgroup_size.len() || dims as usize != grid_size.len() {
            return Err(());
        }
        let mut packet: KernelDispatchPacket = unsafe { zeroed() };
        packet.header = {
            let mut header = (packet_type as u16) << (PacketHeader::Type as u16);
            for &(scope, typ) in fences {
                header |= (scope as u16) << (typ as u16);
            }
            header
        };
        packet.setup = (dims << (KernelDispatchPacketSetup::Dimensions as u8)) as u16;
        packet.workgroup_size_x = workgroup_size[0];
        packet.grid_size_x = grid_size[0];
        packet.workgroup_size_y = 1;
        packet.workgroup_size_z = 1;
        packet.grid_size_y = 1;
        packet.grid_size_z = 1;
        if dims > 1 {
            packet.workgroup_size_y = workgroup_size[1];
            packet.grid_size_y = grid_size[1];
            if dims > 2 {
                packet.workgroup_size_z = workgroup_size[2];
                packet.grid_size_z = grid_size[2];
            }
        }
        packet.private_segment_size = private_segment_size;
        packet.group_segment_size = group_segment_size;
        packet.kernel_object = kernel_object;
        packet.kernarg_address = kernarg_address as *const c_void;
        packet.completion_signal = completion_signal.handle();
        Ok(packet)
    }
}
