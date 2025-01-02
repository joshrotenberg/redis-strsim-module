use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};
use strsim::StrSimError;

/// STRSIM.HAMMING <first> <second>
///
/// Computes the Hamming distance between two strings.
pub(crate) fn hamming(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    match strsim::hamming(first, second) {
        Ok(distance) => Ok(RedisValue::Integer(distance as i64)),
        Err(StrSimError::DifferentLengthArgs) => {
            Err(RedisError::Str("ERR Arguments must be of the same length"))
        }
    }
}
