use talc::*;

static mut ARENA: [u8; 100000] = [0; 100000];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    // if we're in a hosted environment, the Rust runtime may allocate before
    // main() is called, so we need to initialize the arena automatically
    ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut()))
}).lock();