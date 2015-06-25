# rust-hsa
HSA (Heterogeneous System Architecture) bindings for Rust

## Requirements

* AMD Kaveri APUs
* [HSA-Drivers-Linux-AMD](https://github.com/HSAFoundation/HSA-Drivers-Linux-AMD)
* [HSA-Runtime-AMD](https://github.com/HSAFoundation/HSA-Runtime-AMD)

## Tested Environment

* CPU: AMD A10-7850K (Kaveri)
* M/B: [MSI A88XI AC](http://www.msi.com/product/mb/A88XI_AC.html) (BIOS: v1.6)
* Linux Distribution: Gentoo Linux (amd64)
* HSA Driver: amdkfd v1.4 (linux 4.0.0+)

## included tools

### hsainfo

Dump HSA information

```
$ cargo run --release --bin hsainfo
     Running `target/release/hsainfo`
HSA SystemInfo
* version: 1.0
* timestamp: 2141.220068967 [s] (freq: 1000 [MHz])
* signal-max-wait: 18446744073709551615
* endianness: Little
* machine_model: Large

AgentInfo
* name: Kaveri CPU
* vendor: AMD
* feature: AgentDispatch
* machine_model: Large
* profile: Full
* float_rouding: Near
* base_profile_float_rouding: Near
* fast_f16_operation: false
* wavefront size: 0
* workgroup max dim: [0, 0, 0]
* workgroup max size: 0
* grid max dim: Dim3 { x: 0, y: 0, z: 0 }
* grid max size: 0
* fbarrier max size: 0
* queues max: 0
* queue max size: 0
* queue min size: 0
* queue type: Multi
* node: 0
* device: CPU
* isa: ISA { handle: 0 }
* version: 1.0

| RegionInfo
|  * segment: Global
|  * global flags: {KernArg, FineGrained}
|  * size: 140737488355328
|  * alloc max size: 15642705920
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096

AgentInfo
* name: Spectre
* vendor: AMD
* feature: KernelDispatch
* machine_model: Large
* profile: Full
* float_rouding: Near
* base_profile_float_rouding: Near
* fast_f16_operation: false
* wavefront size: 64
* workgroup max dim: [2048, 2048, 2048]
* workgroup max size: 2048
* grid max dim: Dim3 { x: 4294967295, y: 4294967295, z: 4294967295 }
* grid max size: 4294967295
* fbarrier max size: 32
* queues max: 128
* queue max size: 131072
* queue min size: 4096
* queue type: Multi
* node: 0
* device: GPU
* isa: ISA { handle: 139926947498336 }
* version: 1.0

| RegionInfo
|  * segment: Group
|  * global flags: {}
|  * size: 65536
|  * alloc max size: 0
|  * runtime alloc allowed: false
|  * runtime alloc granule: 0
|  * runtime alloc alignment: 0

| RegionInfo
|  * segment: Global
|  * global flags: {CoarseGrained}
|  * size: 1073741824
|  * alloc max size: 268435456
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096

| RegionInfo
|  * segment: Global
|  * global flags: {FineGrained, KernArg}
|  * size: 140737488355328
|  * alloc max size: 15642705920
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096
```

### hsa-memcpy

memcpy benchmark tool

```
$ cargo run --release --bin hsa-memcpy
     Running `target/release/hsa-memcpy`
memcpy 4 MB => 4653.560998606307 MB/s
memcpy 64 MB => 5898.999940642528 MB/s
memcpy 256 MB => 5791.381699230143 MB/s
memcpy 1024 MB => 6910.0210435532 MB/s
memcpy 4096 MB => 6934.495177298106 MB/s
```
