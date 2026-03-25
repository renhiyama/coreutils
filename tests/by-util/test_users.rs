// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use uutests::new_ucmd;
#[cfg(any(target_vendor = "apple", any(target_os = "linux", target_os = "runixos")))]
use uutests::{util::TestScenario, util_name};

#[ignore = "does not work as same as users > /dev/full"]
#[test]
#[cfg(any(target_os = "linux", target_os = "runixos"))]
fn test_full_panic() {
    let full = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .unwrap();

    new_ucmd!()
        .set_stdout(full)
        .fails()
        .stderr_contains("No space");
}

#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails_with_code(1);
}

#[test]
fn test_users_no_arg() {
    new_ucmd!().succeeds();
}

#[test]
#[cfg(any(target_vendor = "apple", any(target_os = "linux", target_os = "runixos")))]
#[ignore = "issue #3219"]
fn test_users_check_name() {
    #[cfg(any(target_os = "linux", target_os = "runixos"))]
    let util_name = util_name!();
    #[cfg(target_vendor = "apple")]
    let util_name = &format!("g{}", util_name!());

    let expected = TestScenario::new(util_name)
        .cmd(util_name)
        .env("LC_ALL", "C")
        .succeeds()
        .stdout_move_str();

    new_ucmd!().succeeds().stdout_is(&expected);
}

#[test]
#[cfg(target_os = "openbsd")]
fn test_users_check_name_openbsd() {
    new_ucmd!()
        .args(&["openbsd_utmp"])
        .succeeds()
        .stdout_contains("test");
}
