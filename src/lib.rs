#[cfg(not(feature = "integration-tests"))]
use redis_module::{redis_module, Context, RedisString, Status};

#[cfg(not(feature = "integration-tests"))]
mod command;

/// Initialize the module.
#[cfg(not(feature = "integration-tests"))]
fn init(ctx: &Context, _awrgs: &[RedisString]) -> Status {
    ctx.log_notice("Module initialized");

    Status::Ok
}

// Set up the module
#[cfg(not(feature = "integration-tests"))]
redis_module! {
    name: "redisstrsim",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    init: init,
    commands: [
        ["strsim.hamming", command::hamming, "readonly", 0, 0, 0],
    ],
    configurations: [
        i64: [],
        string: [],
        bool: [
        ],
        enum: [
        ],
        module_args_as_configuration: true,
    ]
}
