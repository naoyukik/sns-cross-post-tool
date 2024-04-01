extern crate BlueSky;

#[test]
fn login_test() {
    let blue_sky = BlueSky;
    assert_eq!(blue_sky.login("username", "password"), true);
}
