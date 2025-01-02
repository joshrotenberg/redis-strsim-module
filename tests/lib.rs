use redis::Connection;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::SyncRunner,
    Container, GenericImage,
};

fn new_container() -> Result<GenericImage, Box<dyn std::error::Error + 'static>> {
    let container = GenericImage::new("redis_strsim", "latest")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    Ok(container)
}

fn new_connection(
    container: &Container<GenericImage>,
) -> Result<Connection, Box<dyn std::error::Error + 'static>> {
    let host = container.get_host()?;
    let host_port = container.get_host_port_ipv4(6379)?;
    let url = format!("redis://{}:{}", host, host_port);

    let client = redis::Client::open(url)?;
    let connection = client.get_connection()?;

    Ok(connection)
}

#[test]
fn test_strsim() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let container = new_container()?.start()?;
    let mut connection = new_connection(&container)?;

    let d_l: u32 = redis::cmd("STRSIM.DAMERAU_LEVENSHTEIN")
        .arg("ab")
        .arg("bca")
        .query(&mut connection)?;

    assert_eq!(d_l, 2);

    let h: u32 = redis::cmd("STRSIM.HAMMING")
        .arg("hamming")
        .arg("hammers")
        .query(&mut connection)?;

    assert_eq!(h, 3);

    let j: f64 = redis::cmd("STRSIM.JARO")
        .arg("Friedrich Nietzsche")
        .arg("Jean-Paul Sartre")
        .query(&mut connection)?;

    assert!((0.392 - j).abs() < 0.001);

    let j_w: f64 = redis::cmd("STRSIM.JARO_WINKLER")
        .arg("cheeseburger")
        .arg("cheese fries")
        .query(&mut connection)?;

    assert!((0.866 - j_w).abs() < 0.001);

    let l: u32 = redis::cmd("STRSIM.LEVENSHTEIN")
        .arg("kitten")
        .arg("sitting")
        .query(&mut connection)?;

    assert_eq!(l, 3);

    let n_d_l_1: f64 = redis::cmd("STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN")
        .arg("levenshtein")
        .arg("löwenbräu")
        .query(&mut connection)?;

    assert!((n_d_l_1 - 0.27272).abs() < 0.00001);

    let n_d_l_2: f64 = redis::cmd("STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN")
        .arg("")
        .arg("")
        .query(&mut connection)?;

    assert!((n_d_l_2 - 1.0).abs() < 0.00001);

    let n_d_l_3: f64 = redis::cmd("STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN")
        .arg("")
        .arg("flower")
        .query(&mut connection)?;

    assert!(n_d_l_3.abs() < 0.00001);

    let n_d_l_4: f64 = redis::cmd("STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN")
        .arg("tree")
        .arg("")
        .query(&mut connection)?;

    assert!(n_d_l_4.abs() < 0.00001);

    let n_d_l_5: f64 = redis::cmd("STRSIM.NORMALIZED_DAMERAU_LEVENSHTEIN")
        .arg("sunglasses")
        .arg("sunglasses")
        .query(&mut connection)?;

    assert!((n_d_l_5 - 1.0).abs() < 0.00001);

    let n_l_1: f64 = redis::cmd("STRSIM.NORMALIZED_LEVENSHTEIN")
        .arg("kitten")
        .arg("sitting")
        .query(&mut connection)?;

    assert!((n_l_1 - 0.5714285714285714).abs() < 0.00001);

    let n_l_2: f64 = redis::cmd("STRSIM.NORMALIZED_LEVENSHTEIN")
        .arg("")
        .arg("")
        .query(&mut connection)?;

    assert!((n_l_2 - 1.0).abs() < 0.00001);

    let n_l_3: f64 = redis::cmd("STRSIM.NORMALIZED_LEVENSHTEIN")
        .arg("")
        .arg("second")
        .query(&mut connection)?;

    assert!(n_l_3.abs() < 0.00001);

    let n_l_4: f64 = redis::cmd("STRSIM.NORMALIZED_LEVENSHTEIN")
        .arg("first")
        .arg("")
        .query(&mut connection)?;

    assert!(n_l_4.abs() < 0.00001);

    let n_l_5: f64 = redis::cmd("STRSIM.NORMALIZED_LEVENSHTEIN")
        .arg("string")
        .arg("string")
        .query(&mut connection)?;

    assert!((n_l_5 - 1.0).abs() < 0.00001);

    let o_d: u32 = redis::cmd("STRSIM.OSA_DISTANCE")
        .arg("ab")
        .arg("bca")
        .query(&mut connection)?;

    assert_eq!(o_d, 3);

    let s_d_1: f64 = redis::cmd("STRSIM.SORENSEN_DICE")
        .arg("")
        .arg("")
        .query(&mut connection)?;

    assert_eq!(s_d_1, 1.0);

    let s_d_2: f64 = redis::cmd("STRSIM.SORENSEN_DICE")
        .arg("")
        .arg("a")
        .query(&mut connection)?;

    assert_eq!(s_d_2, 0.0);

    let s_d_3: f64 = redis::cmd("STRSIM.SORENSEN_DICE")
        .arg("french")
        .arg("quebec")
        .query(&mut connection)?;

    assert_eq!(s_d_3, 0.0);

    let s_d_4: f64 = redis::cmd("STRSIM.SORENSEN_DICE")
        .arg("ferris")
        .arg("ferris")
        .query(&mut connection)?;

    assert_eq!(s_d_4, 1.0);

    let s_d_5: f64 = redis::cmd("STRSIM.SORENSEN_DICE")
        .arg("feris")
        .arg("ferris")
        .query(&mut connection)?;

    assert_eq!(s_d_5, 0.8888888888888888);

    Ok(())
}
