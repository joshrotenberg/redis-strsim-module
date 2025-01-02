use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};
use strsim::StrSimError;

/// STRSIM.DAMERAU_LEVENSHTEIN <a> <b>
///
/// Computes the Damerau-Levenshtein distance between two strings.
pub(crate) fn damerau_levenshtein(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(strsim::damerau_levenshtein(a, b) as i64))
}

/// STRSIM.HAMMING <a> <b>
///
/// Computes the Hamming distance between two strings.
pub(crate) fn hamming(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    match strsim::hamming(a, b) {
        Ok(distance) => Ok(RedisValue::Integer(distance as i64)),
        Err(StrSimError::DifferentLengthArgs) => {
            Err(RedisError::Str("ERR Arguments must be of the same length"))
        }
    }
}

/// STRSIM.JARO <a> <b>
///
/// Computes the Jaro distance between two strings.
pub(crate) fn jaro(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::jaro(a, b)))
}

/// STRSIM.JARO_WINKLER <a> <b>
///
/// Computes the Jaro-Winkler distance between two strings.
pub(crate) fn jaro_winkler(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::jaro_winkler(a, b)))
}

/// STRSIM.LEVENSHTEIN <a> <b>
///
/// Computes the Levenshtein distance between two strings.
pub(crate) fn levenshtein(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(strsim::levenshtein(a, b) as i64))
}

/// STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN <a> <b>
///
/// Computes the normalized Damerau-Levenshtein distance between two strings.
pub(crate) fn normalized_damerau_levenshtein(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::normalized_damerau_levenshtein(
        a, b,
    )))
}

/// STRSIM.NORMALIZED_HAMMING <a> <b>
///
/// Computes the normalized Hamming distance between two strings.
pub(crate) fn normalized_levenshtein(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::normalized_levenshtein(a, b)))
}

/// STRSIM.OSA_DISTANCE <a> <b>
///
/// Computes the Optimal String Alignment distance between two strings.
pub(crate) fn osa_distance(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(strsim::osa_distance(a, b) as i64))
}

/// STRSIM.SORENSEN_DICE <a> <b>
///
/// Computes the Sørensen-Dice coefficient between two strings.
pub(crate) fn sorensen_dice(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let a = args.next_arg()?.try_as_str()?;
    let b = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::sorensen_dice(a, b)))
}
