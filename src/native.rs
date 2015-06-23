use libc::{c_char, c_int, c_void, size_t};
pub type SignalValue = i64;

#[link(name = "hsa-runtime64")]
extern {
    // 2.1 Initialization and Shut Down
    pub fn hsa_init() -> c_int;
    pub fn hsa_shut_down() -> c_int;

    // 2.3 System and Agent Information
    pub fn hsa_system_get_info(attribute: SystemInfo,
                               value: *mut c_void) -> c_int;
    pub fn hsa_agent_get_info(agent: Agent,
                              attribute: AgentInfo,
                              value: *mut c_void) -> c_int;
    pub fn hsa_iterate_agents(callback: extern fn(Agent, *mut c_void) -> c_int,
                              data: *mut c_void) -> c_int;

    // 2.4 Signals
    pub fn hsa_signal_create(initial_value: SignalValue,
                             num_consumers: u32,
                             consumers: *const Agent,
                             signal: &SignalHandle) -> c_int;
    pub fn hsa_signal_destroy(signal: SignalHandle) -> c_int;
    pub fn hsa_signal_load_acquire(signal: SignalHandle) -> SignalValue;
    pub fn hsa_signal_load_relaxed(signal: SignalHandle) -> SignalValue;
    pub fn hsa_signal_store_relaxed(signal: SignalHandle,
                                    value: SignalValue);
    pub fn hsa_signal_store_release(signal: SignalHandle,
                                    value: SignalValue);
    pub fn hsa_signal_exchange_acq_rel(signal: SignalHandle,
                                       value: SignalValue) -> SignalValue;
    pub fn hsa_signal_exchange_acquire(signal: SignalHandle,
                                       value: SignalValue) -> SignalValue;
    pub fn hsa_signal_exchange_relaxed(signal: SignalHandle,
                                       value: SignalValue) -> SignalValue;
    pub fn hsa_signal_exchange_release(signal: SignalHandle,
                                       value: SignalValue) -> SignalValue;
    pub fn hsa_signal_cas_acq_rel(signal: SignalHandle,
                                  expected: SignalValue,
                                  value: SignalValue) -> SignalValue;
    pub fn hsa_signal_cas_acquire(signal: SignalHandle,
                                  expected: SignalValue,
                                  value: SignalValue) -> SignalValue;
    pub fn hsa_signal_cas_relaxed(signal: SignalHandle,
                                  expected: SignalValue,
                                  value: SignalValue) -> SignalValue;
    pub fn hsa_signal_cas_release(signal: SignalHandle,
                                  expected: SignalValue,
                                  value: SignalValue) -> SignalValue;
    pub fn hsa_signal_add_acq_rel(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_add_acquire(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_add_relaxed(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_add_release(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_subtract_acq_rel(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_subtract_acquire(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_subtract_relaxed(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_subtract_release(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_and_acq_rel(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_and_acquire(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_and_relaxed(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_and_release(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_or_acq_rel(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_or_acquire(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_or_relaxed(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_or_release(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_xor_acq_rel(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_xor_acquire(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_xor_relaxed(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_xor_release(signal: SignalHandle, value: SignalValue);
    pub fn hsa_signal_wait_acquire(signal: SignalHandle,
                                   condition: SignalCondition,
                                   compare_value: SignalValue,
                                   timeout_hint: u64,
                                   wait_state_hint: WaitState) -> SignalValue;
    pub fn hsa_signal_wait_relaxed(signal: SignalHandle,
                                   condition: SignalCondition,
                                   compare_value: SignalValue,
                                   timeout_hint: u64,
                                   wait_state_hint: WaitState) -> SignalValue;
    

    // 2.5 Queues
    pub fn hsa_queue_create(agent: Agent,
                            size: u32,
                            queue_type: QueueType,
                            callback: *const c_void,
                            data: *const c_void,
                            private_segment_size: u32,
                            group_segment_size: u32,
                            queue: &*const QueueHandle) -> c_int;
    pub fn hsa_queue_destroy(queue: *const QueueHandle) -> c_int;
    pub fn hsa_queue_load_read_index_acquire(queue: *const QueueHandle) -> u64;
    pub fn hsa_queue_load_read_index_relaxed(queue: *const QueueHandle) -> u64;
    pub fn hsa_queue_load_write_index_acquire(queue: *const QueueHandle) -> u64;
    pub fn hsa_queue_load_write_index_relaxed(queue: *const QueueHandle) -> u64;
    pub fn hsa_queue_store_write_index_relaxed(queue: *const QueueHandle, value: u64);
    pub fn hsa_queue_store_write_index_release(queue: *const QueueHandle, value: u64);
    pub fn hsa_queue_cas_write_index_acq_rel(queue: *const QueueHandle,
                                             expected: u64, value: u64) -> u64;
    pub fn hsa_queue_cas_write_index_acquire(queue: *const QueueHandle,
                                             expected: u64, value: u64) -> u64;
    pub fn hsa_queue_cas_write_index_relaxed(queue: *const QueueHandle,
                                             expected: u64, value: u64) -> u64;
    pub fn hsa_queue_cas_write_index_release(queue: *const QueueHandle,
                                             expected: u64, value: u64) -> u64;
    pub fn hsa_queue_add_write_index_acq_rel(queue: *const QueueHandle,
                                             value: u64) -> u64;
    pub fn hsa_queue_add_write_index_acquire(queue: *const QueueHandle,
                                             value: u64) -> u64;
    pub fn hsa_queue_add_write_index_relaxed(queue: *const QueueHandle,
                                             value: u64) -> u64;
    pub fn hsa_queue_add_write_index_release(queue: *const QueueHandle,
                                             value: u64) -> u64;
    pub fn hsa_queue_store_read_index_relaxed(queue: *const QueueHandle,
                                              value: u64);
    pub fn hsa_queue_store_read_index_release(queue: *const QueueHandle,
                                              value: u64);

    // 2.7 Memory
    pub fn hsa_region_get_info(region: Region,
                               attribute: RegionInfo,
                               value: *mut c_void) -> c_int;
    pub fn hsa_agent_iterate_regions(agent: Agent,
                                     callback: extern fn(region: Region,
                                                         data: *mut c_void) -> c_int,
                                     data: *mut c_void) -> c_int;
    pub fn hsa_memory_allocate(region: Region,
                               size: size_t,
                               ptr: &*mut c_void) -> c_int;
    pub fn hsa_memory_free(ptr: *mut c_void) -> c_int;
    pub fn hsa_memory_copy(dst: *mut c_void,
                           src: *const c_void,
                           size: size_t) -> c_int;
    pub fn hsa_memory_assign_agent(ptr: *mut c_void,
                                   agent: Agent,
                                   access: AccessPermission) -> c_int;
    pub fn hsa_memory_register(ptr: *mut c_void, size: size_t) -> c_int;
    pub fn hsa_memory_deregister(ptr: *mut c_void, size: size_t) -> c_int;

    // 2.8 Code Objects and Executables
    pub fn hsa_executable_create(profile: Profile,
                                 executable_state: ExecutableState,
                                 options: *const c_char,
                                 executable: &ExecutableHandle) -> c_int;
    pub fn hsa_executable_destroy(executable: ExecutableHandle) -> c_int;
    pub fn hsa_executable_load_code_object(executable: ExecutableHandle,
                                           agent: Agent,
                                           code_object: CodeObject,
                                           options: *const c_char) -> c_int;
    pub fn hsa_executable_freeze(executable: ExecutableHandle,
                                 options: *const c_char) -> c_int;
    pub fn hsa_executable_get_symbol(executable: ExecutableHandle,
                                     module_name: *const c_char,
                                     symbol_name: *const c_char,
                                     agent: Agent,
                                     call_convention: i32,
                                     symbol: &ExecutableSymbol) -> c_int;
    pub fn hsa_executable_symbol_get_info(executable_symbol: ExecutableSymbol,
                                          attribute: ExecutableSymbolInfo,
                                          value: *mut c_void) -> c_int;

    // 3.2.1 HSAIL Finalization API
    pub fn hsa_ext_program_create(machine_model: MachineModel,
                                  profile: Profile,
                                  default_float_rouding_mode: DefaultFloatRoundingMode,
                                  options: *const c_char,
                                  program: &ProgramHandle) -> c_int;
    pub fn hsa_ext_program_destroy(program: ProgramHandle) -> c_int;
    pub fn hsa_ext_program_add_module(program: ProgramHandle,
                                      module: *const c_void) -> c_int;
    pub fn hsa_ext_program_finalize(program: ProgramHandle,
                                    isa: ISA,
                                    call_convention: i32,
                                    controll_directives: ControlDirectives,
                                    options: *const c_char,
                                    code_object_type: CodeObjectType,
                                    code_object: &CodeObject) -> c_int;
}


#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum Endianness {
    Little = 0,
    Big = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum MachineModel {
    Small = 0,
    Large = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum Profile {
    Base = 0,
    Full = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum SystemInfo {
    VersionMajor = 0,
    VersionMinor = 1,
    Timestamp = 2,
    TimestampFrequency = 3,
    SignalMaxWait = 4,
    Endianness = 5,
    MachineModel = 6,
    Extensions = 7
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum Extension {
    Finalizer = 0,
    Images = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum AgentFeature {
    KernelDispatch = 1,
    AgentDispatch = 2,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum DeviceType {
    CPU = 0,
    GPU = 1,
    DSP = 2,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum DefaultFloatRoundingMode {
    Default = 0,
    Zero = 1,
    Near = 2,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum AgentInfo {
    Name = 0,
    VendorName = 1,
    Feature = 2,
    MachineModel = 3,
    Profile = 4,
    DefaultFloatRoundingMode = 5,
    BaseProfileDefaultFloatRoundingModes = 23,
    FastF16Operation = 24,
    WavefrontSize = 6,
    WorkgroupMaxDim = 7,
    WorkgroupMaxSize = 8,
    GridMaxDim = 9,
    GridMaxSize = 10,
    FbarrierMaxSize = 11,
    QueuesMax = 12,
    QueueMinSize = 13,
    QueueMaxSize = 14,
    QueueType = 15,
    Node = 16,
    Device = 17,
    CacheSize = 18,
    ISA = 19,
    Extensions = 20,
    VersionMajor = 21,
    VersionMinor = 22,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ExceptionPolicy {
    Break = 1,
    Detect = 2,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum QueueType {
    Multi = 0,
    Single = 1,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Dim3 {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ISA {
    handle: u64,
}

#[derive(Clone)]
#[repr(C)]
pub struct Agent {
    handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum QueueFeature {
    KernelDispatch = 1,
    AgentDispatch = 2,
}

#[repr(C)]
pub struct QueueHandle {
    pub queue_type: QueueType,
    pub base_address: *mut c_void,
    pub doorbell_signal: SignalHandle,
    pub size: u32,
    reserved1: u32,
    pub id: u64,
}

#[derive(Clone)]
#[repr(C)]
pub struct Region {
    handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum RegionSegment {
    Global = 0,
    ReadOnly = 1,
    Private = 2,
    Group = 3,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
#[repr(C)]
pub enum RegionGlobalFlag {
    KernArg = 1,
    FineGrained = 2,
    CoarseGrained = 4,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum RegionInfo {
    Segment = 0,
    GlobalFlags = 1,
    Size = 2,
    AllocMaxSize = 4,
    RuntimeAllocAllowed = 5,
    RuntimeAllocGranule = 6,
    RuntimeAllocAlignment = 7,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum AccessPermission {
    RO = 1,
    WO = 2,
    RW = 3,
}

#[derive(Clone)]
#[repr(C)]
pub struct ProgramHandle {
    pub handle: u64,
}

#[derive(Clone)]
#[repr(C)]
pub struct ControlDirectives {
    control_directives_mask: u64,
    break_exceptions_mask: u16,
    detect_exceptions_mask: u16,
    max_dynamic_group_size: u32,
    max_flat_grid_size: u64,
    max_flat_workgroup_size: u32,
    reserved1: u32,
    required_grid_size: [u64; 3],
    required_workgroup_size: Dim3,
    required_dim: u8,
    reserved2: [u64; 10],
    //reserved2: [u8; 75],
}

#[derive(Clone)]
#[repr(C)]
pub struct CodeObject {
    handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum CodeObjectType {
    Program = 0,
    Dummy,
}

#[derive(Clone)]
#[repr(C)]
pub struct ExecutableHandle {
    pub handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ExecutableState {
    Unfrozen = 0,
    Frozen = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ExecutableInfo {
    Profile = 1,
    State = 2,
}

#[derive(Clone)]
#[repr(C)]
pub struct ExecutableSymbol {
    handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ExecutableSymbolInfo {
    Type = 0,
    NameLength = 1,
    Name = 2,
    ModuleNameLength = 3,
    ModuleName = 4,
    Agent = 20,
    VariableAddress = 21,
    Linkage = 5,
    IsDefinition = 17,
    VariableAllocation = 6,
    VariableSegment = 7,
    VariableAlignment = 8,
    VariableSize = 9,
    VariableIsConst = 10,
    KernelObject = 22,
    KernelKernArgSegmentSize = 11,
    KernelKernArgSegmentAlignment = 12,
    KernelGroupSegmentSize = 13,
    KernelPrivateSegmentSize = 14,
    KernelDynamicCallstack = 15,
    IndirectFunctionObject = 23,
    IndirectFunctionCallConvention = 16,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum SymbolKind {
    Variable = 0,
    Kernel = 1,
    IndirectFunction = 2,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum VariableAllocation {
    Agent = 0,
    Program = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum SymbolKindLinkage {
    Module = 0,
    Program = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum VariableSegment {
    Global = 0,
    ReadOnly = 1,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ISAInfo {
    NameLength = 0,
    Name = 1,
    CallConventionCount = 2,
    CallConventionInfoWaveFrontSize = 3,
    CallConventionInfoWaveFrontsPerComputeUnit = 4,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum CodeObjectInfo {
    Version = 0,
    Type = 1,
    ISA = 2,
    MachineModel = 3,
    Profile = 4,
    DefaultFloatRoudingMode = 5,
}

#[derive(Clone)]
#[repr(C)]
pub struct SignalHandle {
    pub handle: u64,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum SignalCondition {
    Eq = 0,
    NE = 1,
    LT = 2,
    GTE = 3,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum WaitState {
    Blocked = 0,
    Active = 1,
}

pub static PACKET_SIZE: usize = 64;

#[repr(C)]
pub struct KernelDispatchPacket {
    header: u16,
    setup: u16,
    workgroup_size_x: u16,
    workgroup_size_y: u16,
    workgroup_size_z: u16,
    reserved0: u16,
    grid_size_x: u32,
    grid_size_y: u32,
    grid_size_z: u32,
    private_segment_size: u32,
    group_segment_size: u32,
    kernel_object: u64,
    kernarg_address: *const c_void,
    reserved2: u64,
    completion_signal: SignalHandle,
}

impl KernelDispatchPacket {
    pub fn new(header: &PacketHeader, setup: &KernelDispatchPacketSetup,
               workgroup_size: &[u16; 3],
               grid_size: &[u32; 3],
               private_segment_size: u32,
               group_segment_size: u32,
               kernel_object: u64,
               kernarg_address: *const c_void,
               completion_signal: SignalHandle) -> KernelDispatchPacket {
        KernelDispatchPacket {
            header: header.to_u16(),
            setup: setup.to_u16(),
            workgroup_size_x: workgroup_size[0],
            workgroup_size_y: workgroup_size[1],
            workgroup_size_z: workgroup_size[2],
            reserved0: 0,
            grid_size_x: grid_size[0],
            grid_size_y: grid_size[1],
            grid_size_z: grid_size[2],
            private_segment_size: private_segment_size,
            group_segment_size: group_segment_size,
            kernel_object: kernel_object,
            kernarg_address: kernarg_address,
            reserved2: 0,
            completion_signal: completion_signal,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub enum PacketType {
    VendorSpecific = 0,
    Invalid = 1,
    KernelDispatch = 2,
    BarrierAnd = 3,
    AgentDispatch = 4,
    BarrierOr = 5,
}

#[derive(PartialEq, Debug, Clone)]
#[repr(C)]
pub enum FenceScope {
    None = 0,
    Agent = 1,
    System = 2,
}

pub struct PacketHeader {
    pub header_type: PacketType,
    pub barrier: bool,
    pub acquire_fence_scope: FenceScope,
    pub release_fence_scope: FenceScope,
}

impl PacketHeader {
    pub fn to_u16(&self) -> u16 {
        (self.header_type.clone() as u16) |
        (match self.barrier {
            true => 1,
            false => 0,
        }) |
        ((self.acquire_fence_scope.clone() as u16) << 9) |
        ((self.release_fence_scope.clone() as u16) << 11)
    }
}

pub struct KernelDispatchPacketSetup {
    pub dimensions: u8,
}

impl KernelDispatchPacketSetup {
    pub fn to_u16(&self) -> u16 {
        (self.dimensions & 0x3) as u16
    }
}
