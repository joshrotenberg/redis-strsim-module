#[cfg(not(feature = "integration-tests"))]
use redis_module::{redis_module, Context, RedisString, Status};

#[cfg(not(feature = "integration-tests"))]
mod command;

#[cfg(not(feature = "integration-tests"))]
fn init(ctx: &Context, _awrgs: &[RedisString]) -> Status {
    ctx.log_notice("Module initialized");

    Status::Ok
}

#[cfg(not(feature = "integration-tests"))]
redis_module! {
    name: "redisstrsim",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    init: init,
    commands: [
        ["strsim.damerau_levenshtein", command::damerau_levenshtein, "readonly", 0, 0, 0],
        ["strsim.hamming", command::hamming, "readonly", 0, 0, 0],
        ["strsim.jaro", command::jaro, "readonly", 0, 0, 0],
        ["strsim.jaro_winkler", command::jaro_winkler, "readonly", 0, 0, 0],
        ["strsim.levenshtein", command::levenshtein, "readonly", 0, 0, 0],
        ["strsim.normalized_damerau_levenshtein", command::normalized_damerau_levenshtein, "readonly", 0, 0, 0],
        ["strsim.normalized_levenshtein", command::normalized_levenshtein, "readonly", 0, 0, 0],
        ["strsim.osa_distance", command::osa_distance, "readonly", 0, 0, 0],
        ["strsim.sorensen_dice", command::sorensen_dice, "readonly", 0, 0, 0],
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
