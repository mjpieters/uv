use std::process::Command;

use assert_fs::{assert::PathAssert, prelude::PathChild};
use predicates::prelude::predicate;

use crate::common::{uv_snapshot, TestContext};

#[test]
fn python_install() {
    let context: TestContext = TestContext::new_with_versions(&[]).with_filtered_python_keys();

    // Install the latest version
    uv_snapshot!(context.filters(), context.python_install(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Installed Python 3.13.0 in [TIME]
     + cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    let bin_python = context
        .temp_dir
        .child("bin")
        .child(format!("python3.13{}", std::env::consts::EXE_SUFFIX));

    // The executable should be installed in the bin directory
    bin_python.assert(predicate::path::exists());

    // On Unix, it should be a link
    #[cfg(unix)]
    bin_python.assert(predicate::path::is_symlink());

    // The executable should "work"
    uv_snapshot!(context.filters(), Command::new(bin_python.as_os_str())
        .arg("-c").arg("import subprocess; print('hello world')"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    hello world

    ----- stderr -----
    "###);

    // Should be a no-op when already installed
    uv_snapshot!(context.filters(), context.python_install(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Found: cpython-3.13.[X]-[PLATFORM]
    Python is already available. Use `uv python install <request>` to install a specific version.
    "###);

    // Similarly, when a requested version is already installed
    uv_snapshot!(context.filters(), context.python_install().arg("3.13"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.13
    Found existing installation for Python 3.13: cpython-3.13.[X]-[PLATFORM]
    "###);

    // You can opt-in to a reinstall
    uv_snapshot!(context.filters(), context.python_install().arg("--reinstall"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Found: cpython-3.13.[X]-[PLATFORM]
    Installed Python 3.13.0 in [TIME]
     ~ cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    // The executable should still be present in the bin directory
    bin_python.assert(predicate::path::exists());

    // Uninstallation requires an argument
    uv_snapshot!(context.filters(), context.python_uninstall(), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the following required arguments were not provided:
      <TARGETS>...

    Usage: uv python uninstall <TARGETS>...

    For more information, try '--help'.
    "###);

    uv_snapshot!(context.filters(), context.python_uninstall().arg("3.13"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.13
    Uninstalled Python 3.13.0 in [TIME]
     - cpython-3.13.[X]-[PLATFORM]
    "###);

    // The executable should be removed
    bin_python.assert(predicate::path::missing());
}

#[test]
fn python_install_freethreaded() {
    let context: TestContext = TestContext::new_with_versions(&[]).with_filtered_python_keys();

    // Install the latest version
    uv_snapshot!(context.filters(), context.python_install().arg("3.13t"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.13t
    Installed Python 3.13.0 in [TIME]
     + cpython-3.13.[X]+freethreaded-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    let bin_python = context
        .temp_dir
        .child("bin")
        .child(format!("python3.13t{}", std::env::consts::EXE_SUFFIX));

    // The executable should be installed in the bin directory
    bin_python.assert(predicate::path::exists());

    // On Unix, it should be a link
    #[cfg(unix)]
    bin_python.assert(predicate::path::is_symlink());

    // The executable should "work"
    uv_snapshot!(context.filters(), Command::new(bin_python.as_os_str())
        .arg("-c").arg("import subprocess; print('hello world')"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    hello world

    ----- stderr -----
    "###);

    // Should be distinct from 3.13
    uv_snapshot!(context.filters(), context.python_install().arg("3.13"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.13
    Installed Python 3.13.0 in [TIME]
     + cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    // Should not work with older Python versions
    uv_snapshot!(context.filters(), context.python_install().arg("3.12t"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.12t
    error: No download found for request: cpython-3.12t-[PLATFORM]
    "###);

    uv_snapshot!(context.filters(), context.python_uninstall().arg("--all"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Uninstalled 2 versions in [TIME]
     - cpython-3.13.[X]-[PLATFORM]
     - cpython-3.13.[X]+freethreaded-[PLATFORM]
    "###);
}

#[test]
fn python_install_default() {
    let context: TestContext = TestContext::new_with_versions(&[]).with_filtered_python_keys();

    // Install the latest version
    uv_snapshot!(context.filters(), context.python_install(), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Installed Python 3.13.0 in [TIME]
     + cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    let bin_python_minor = context
        .temp_dir
        .child("bin")
        .child(format!("python3.13{}", std::env::consts::EXE_SUFFIX));

    let bin_python_major = context
        .temp_dir
        .child("bin")
        .child(format!("python3{}", std::env::consts::EXE_SUFFIX));

    let bin_python_default = context
        .temp_dir
        .child("bin")
        .child(format!("python{}", std::env::consts::EXE_SUFFIX));

    // The minor-versioend executable should be installed in the bin directory
    bin_python_minor.assert(predicate::path::exists());
    // But the others should not
    bin_python_major.assert(predicate::path::missing());
    bin_python_default.assert(predicate::path::missing());

    // TODO(zanieb): We should add the executables without the `--reinstall` flag
    uv_snapshot!(context.filters(), context.python_install().arg("--default").arg("--reinstall"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python installations
    Found: cpython-3.13.[X]-[PLATFORM]
    Installed Python 3.13.0 in [TIME]
     ~ cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    "###);

    // Now we should have an unversioned and major-versioned executable
    bin_python_minor.assert(predicate::path::exists());
    bin_python_major.assert(predicate::path::exists());
    bin_python_default.assert(predicate::path::exists());

    uv_snapshot!(context.filters(), context.python_uninstall().arg("3.13"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.13
    Uninstalled Python 3.13.0 in [TIME]
     - cpython-3.13.[X]-[PLATFORM]
    "###);

    // We should remove all the executables
    bin_python_minor.assert(predicate::path::missing());
    bin_python_major.assert(predicate::path::missing());
    bin_python_default.assert(predicate::path::missing());

    // Install multiple versions
    uv_snapshot!(context.filters(), context.python_install().arg("3.12").arg("3.13").arg("--default"), @r###"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Searching for Python versions matching: Python 3.12
    Searching for Python versions matching: Python 3.13
    Installed 2 versions in [TIME]
     + cpython-3.12.[X]-[PLATFORM]
     + cpython-3.13.[X]-[PLATFORM]
    warning: `[TEMP_DIR]/bin` is not on your PATH. To use the installed Python executable, run `export PATH="[TEMP_DIR]/bin:$PATH"`.
    error: Failed to install cpython-3.12.[X]-[PLATFORM]
      Caused by: Executable already exists at `bin/python3`. Use `--reinstall` to force replacement.
    error: Failed to install cpython-3.12.[X]-[PLATFORM]
      Caused by: Executable already exists at `bin/python`. Use `--reinstall` to force replacement.
    "###);

    bin_python_minor.assert(predicate::path::exists());
    bin_python_major.assert(predicate::path::exists());
    bin_python_default.assert(predicate::path::exists());
    // TODO(zanieb): Assert that 3.12 is the default version
}
