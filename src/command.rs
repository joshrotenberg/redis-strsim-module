use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};
use strsim::StrSimError;

/// STRSIM.DAMERAU_LEVENSHTEIN <first> <second>
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
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(
        strsim::damerau_levenshtein(first, second) as i64,
    ))
}

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

/// STRSIM.JARO <first> <second>
///
/// Computes the Jaro distance between two strings.
pub(crate) fn jaro(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::jaro(first, second)))
}

/// STRSIM.JARO_WINKLER <first> <second>
///
/// Computes the Jaro-Winkler distance between two strings.
pub(crate) fn jaro_winkler(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(strsim::jaro_winkler(first, second)))
}

/// STRSIM.LEVENSHTEIN <first> <second>
///
/// Computes the Levenshtein distance between two strings.
pub(crate) fn levenshtein(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(
        strsim::levenshtein(first, second) as i64
    ))
}

/// STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN <first> <second>
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
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(
        strsim::normalized_damerau_levenshtein(first, second)
    ))
}

/// STRSIM.NORMALIZED_HAMMING <first> <second>
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
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(
        strsim::normalized_levenshtein(first, second)
    ))
}

/// STRSIM.OSA_DISTANCE <first> <second>
///
/// Computes the Optimal String Alignment distance between two strings.
pub(crate) fn osa_distance(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Integer(
        strsim::osa_distance(first, second) as i64
    ))
}

/// STRSIM.SORENSEN_DICE <first> <second>
///
/// Computes the Sørensen-Dice coefficient between two strings.
pub(crate) fn sorensen_dice(
    _ctx: &Context,
    args: Vec<RedisString>,
) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let first = args.next_arg()?.try_as_str()?;
    let second = args.next_arg()?.try_as_str()?;

    Ok(RedisValue::Float(
        strsim::sorensen_dice(first, second)
    ))
}