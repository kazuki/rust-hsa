# rust-hsa

Heterogeneous System Architecture(HSA) v1.1 bindings for Rust

## Requirements

* [ROCm compatible GPU + CPU + M/B](https://rocm.github.io/hardware.html)
* [ROCK-Kernel-Driver](https://github.com/RadeonOpenCompute/ROCK-Kernel-Driver)
* [ROCT-Thunk-Interface](https://github.com/RadeonOpenCompute/ROCT-Thunk-Interface)
* [ROCR-Runtime](https://github.com/RadeonOpenCompute/ROCR-Runtime)
* AMD HSAIL Fializer Extension (included in http://repo.radeon.com/rocm/apt/debian/pool/main/h/hsa-ext-rocr-dev/)

## Tested Environment

* 2017/12/25

    * CPU: AMD Threadripper 1950X
    * M/B: [ASRock X399 Taichi](http://www.asrock.com/MB/AMD/X399%20Taichi/index.asp)
    * Linux: Gentoo Linux (amd64)
    * ROCm: v1.7.0

* 2017/07/19

    * CPU: AMD Ryzen 7 1800X
    * M/B: [ASRock AB350M Pro](http://www.asrock.com/mb/AMD/AB350M%20Pro4/index.asp)
    * Linux Distribution: Gentoo Linux (amd64)
    * ROCm: v1.6.0

## included tools

### hsainfo

Dump HSA information

```
$ cargo run --bin hsainfo
[HSA SystemInfo]
* version: 1.1
* timestamp: 213889.619526029 [s] (freq: 1000 [MHz])
* signal-max-wait: 18446744073709551615
* endianness: Little
* machine_model: Large
* extensions: ["HSA_EXTENSION_FINALIZER"]

[Agent (CPU)]
  * name: AMD Ryzen Threadripper 1950X 16-Core Processor
  * vendor: CPU
  * feature: AgentDispatch
  * machine_model/profile: Large / Full (deprecated)
  * float_rounding: Near / Near (deprecated)
  * fast_f16_operation: false (deprecated)
  * wavefront size: 0 (deprecated)
  * workgroup max dim: [0, 0, 0] (deprecated)
  * workgroup max size: 0 (deprecated)
  * grid max dim: Dim3 { x: 0, y: 0, z: 0 } (deprecated)
  * grid max size: 0 (deprecated)
  * fbarrier max size: 0 (deprecated)
  * queues max: 0 (deprecated)
  * queue min/max/type: 0 / 0 / Multi
  * version: 1.1
  [Cache]
    * AMD Ryzen Threadripper 1950X 16-Core Processor L1(L1 32768KiB) x 16
  [Region (Global)]
    * size: 33691586560
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096
  [Region (Global)]
    * size: 33691586560
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096

[Agent (CPU)]
  * name: AMD Ryzen Threadripper 1950X 16-Core Processor
  * vendor: CPU
  * feature: AgentDispatch
  * machine_model/profile: Large / Full (deprecated)
  * float_rounding: Near / Near (deprecated)
  * fast_f16_operation: false (deprecated)
  * wavefront size: 0 (deprecated)
  * workgroup max dim: [0, 0, 0] (deprecated)
  * workgroup max size: 0 (deprecated)
  * grid max dim: Dim3 { x: 0, y: 0, z: 0 } (deprecated)
  * grid max size: 0 (deprecated)
  * fbarrier max size: 0 (deprecated)
  * queues max: 0 (deprecated)
  * queue min/max/type: 0 / 0 / Multi
  * version: 1.1
  [Cache]
    * AMD Ryzen Threadripper 1950X 16-Core Processor L1(L1 32768KiB) x 8
  [Region (Global)]
    * size: 33691586560
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096
  [Region (Global)]
    * size: 33691586560
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096

[Agent (GPU)]
  * name: gfx803
  * vendor: AMD
  * feature: KernelDispatch
  * machine_model/profile: Large / Base (deprecated)
  * float_rounding: Near / Near (deprecated)
  * fast_f16_operation: false (deprecated)
  * wavefront size: 64 (deprecated)
  * workgroup max dim: [1024, 1024, 1024] (deprecated)
  * workgroup max size: 1024 (deprecated)
  * grid max dim: Dim3 { x: 4294967295, y: 4294967295, z: 4294967295 } (deprecated)
  * grid max size: 4294967295 (deprecated)
  * fbarrier max size: 32 (deprecated)
  * queues max: 128 (deprecated)
  * queue min/max/type: 4096 / 131072 / Multi
  * version: 1.1
  [Cache]
    * gfx803 L1(L1 16KiB) x 64
    * gfx803 L1(L1 4KiB) x 16
  [ISA]
    * name: AMD:AMDGPU:8:0:3�
    * machine_models/profiles: {Large} / {Base}
    * float rounding modes: {Near} / {Near}
    * fast f16 ops: true
    * workgroup max dim: [1024, 1024, 1024]
    * workgroup max size: 1024
    * grid max dim: Dim3 { x: 4294967295, y: 4294967295, z: 4294967295 }
    * grid max size: 18446744073709551615
    * fbarrier max size: 32
    [Wavefront]
      * size: 64
  [Region (Global)]
    * size: 4294967296
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 4294967296 4096 4096
  [Region (Group)]
    * size: 65536
  [Region (Global)]
    * size: 33691586560
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {KernArg, FineGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096
  [Region (Global)]
    * size: 33691586560
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33691586560 4096 4096
  [Region (Global)]
    * size: 33800904704
    * global flags: {CoarseGrained}
    * alloc max_size/granule/align: 33800904704 4096 4096
```

### vector-copy

HSA sample program

```
$ cargo run --bin vector_copy
Checking finalizer 1.0 extension support succeeded.
Getting a gpu agent succeeded.
Querying the agent name succeeded.
The agent name is gfx803.
Getting a ISA succeeded.
Obtaining machine model succeeded.
Querying the isa maximum queue size succeeded.
The maximum queue size is 131072.
Creating the queue succeeded.
Create the program succeeded.
Adding the brig module to the program succeeded.
Finalizing the program succeeded.
Create the executable succeeded.
Loading the code object succeeded.
Freeze the executable succeeded.
Extract the symbol from the executable succeeded.
Extracting the kernel object from the executable succeeded.
Extracting the group segment size from the executable succeeded.
Extracting the private segment size from the executable succeeded.
Creating a HSA signal succeeded.
Finding a fine grained memory region succeeded.
Finding a kernarg memory region succeeded.
Allocating argument memory for input parameter succeeded.
Allocating argument memory for output parameter succeeded.
Allocating kernel argument memory buffer succeeded.
Dispatching the kernel
Passed validation.
```
