├── blocking_mutex 
│   ├── mod.rs - thread_mode_mutex conditional Compilation []
│   └── raw.rs - thread_mode module conditional compilation []
├── channel
│   ├── channel.rs - [X]
│   ├── mod.rs - [X]
│   └── signal.rs - [X]
├── executor
│   ├── arch
│   │   ├── cortex_m.rs 
- understand usage of asm::sev() in initialization of executor []
- port usage of NVIC !IMPORTANT []
    - how does pend() command work, is it easily portable to riscv peripheral access crate?
    - 

│   │   ├── std.rs - not necessary [X]
│   │   └── wasm.rs - not necessary [X]
        + ##Add `riscv.rs` 
│   ├── mod.rs - Add conditional compilation to use `riscv.rs` []
│   ├── raw
│   │   ├── mod.rs
│   │   ├── run_queue.rs - Make sure AtomicPointer is supported []
│   │   ├── timer_queue.rs - Make sure AtomicPointer is supported []
│   │   ├── util.rs - nothing [X]
│   │   └── waker.rs - Understand rawWaker from core library, no porting necessary [X]
│   └── spawner.rs - no porting necessary
├── fmt.rs
├── interrupt.rs - has several uses of cortex-m
- Split InterruptEXT to trait and impls
- implement InterruptEXT trait for riscV arch using PAC


├── io - looks good! [X]
│   ├── error.rs
│   ├── mod.rs
│   ├── std.rs
│   ├── traits.rs
│   └── util
│       ├── copy_buf.rs
│       ├── drain.rs
│       ├── flush.rs
│       ├── mod.rs
│       ├── read.rs
│       ├── read_buf.rs
│       ├── read_byte.rs
│       ├── read_exact.rs
│       ├── read_to_end.rs
│       ├── read_while.rs
│       ├── skip_while.rs
│       ├── split.rs
│       ├── write.rs
│       ├── write_all.rs
│       └── write_byte.rs
├── lib.rs
├── mutex.rs
├── time - IMPORTANT, need to have a working time driver for ESP32
│   ├── delay.rs
│   ├── driver.rs - 
- [] create Driver struct 
- [] implement Driver for it 
- [] Register it as the global driver with [`time_driver_impl`].
- [] Enable the Cargo features `embassy/time` and one of `embassy/time-tick-*` corresponding to the
│   ├── driver_std.rs -- irrelevant
│   ├── driver_wasm.rs -- irrelevant
│   ├── duration.rs
│   ├── instant.rs
│   ├── mod.rs
│   └── timer.rs
├── util
│   ├── forever.rs
│   ├── mod.rs
│   ├── select_all.rs
│   └── yield_now.rs
└── waitqueue
    ├── mod.rs
    ├── waker.rs
    └── waker_agnostic.rs