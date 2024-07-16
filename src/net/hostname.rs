#[cfg(all(any(unix, windows), feature = "std"))]
pub fn gethostname() -> std::ffi::OsString {
    gethostname_impl()
}

#[cfg(all(unix, feature = "std"))]
#[inline]
fn gethostname_impl() -> std::ffi::OsString {
    use libc::{c_char, sysconf, _SC_HOST_NAME_MAX};
    use std::os::unix::ffi::OsStringExt;
    // Get the maximum size of host names on this system, and account for the
    // trailing NUL byte.
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
    let mut buffer = vec![0; (hostname_max as usize) + 1];
    let returncode = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut c_char, buffer.len()) };
    if returncode != 0 {
        // There are no reasonable failures, so lets panic
        panic!(
            "gethostname failed: {}
    Please report an issue to <https://github.com/swsnr/gethostname.rs/issues>!",
            std::io::Error::last_os_error()
        );
    }
    // We explicitly search for the trailing NUL byte and cap at the buffer
    // length: If the buffer's too small (which shouldn't happen since we
    // explicitly use the max hostname size above but just in case) POSIX
    // doesn't specify whether there's a NUL byte at the end, so if we didn't
    // check we might read from memory that's not ours.
    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    buffer.resize(end, 0);
    std::ffi::OsString::from_vec(buffer)
}

#[cfg(all(windows, feature = "std"))]
#[inline]
fn gethostname_impl() -> OsString {
    use std::os::windows::ffi::OsStringExt;

    // The DNS host name of the local computer. If the local computer is a node
    // in a cluster, lpBuffer receives the DNS host name of the local computer,
    // not the name of the cluster virtual server.
    pub const COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME: i32 = 5;

    // https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw
    ::windows_targets::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype: i32, lpbuffer: *mut u16, nsize: *mut u32) -> i32);

    let mut buffer_size: u32 = 0;

    unsafe {
        // This call always fails with ERROR_MORE_DATA, because we pass NULL to
        // get the required buffer size.  GetComputerNameExW then fills buffer_size with the size
        // of the host name string plus a trailing zero byte.
        GetComputerNameExW(
            COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
            std::ptr::null_mut(),
            &mut buffer_size,
        )
    };
    assert!(
        0 < buffer_size,
        "GetComputerNameExW did not provide buffer size"
    );

    let mut buffer = vec![0_u16; buffer_size as usize];
    unsafe {
        if GetComputerNameExW(
            COMPUTER_NAME_PHYSICAL_DNS_HOSTNAME,
            buffer.as_mut_ptr(),
            &mut buffer_size,
        ) == 0
        {
            panic!(
                "GetComputerNameExW failed to read hostname.
        Please report this issue to <https://github.com/swsnr/gethostname.rs/issues>!"
            );
        }
    }
    assert!(
        // GetComputerNameExW returns the size _without_ the trailing zero byte on the second call
        buffer_size as usize == buffer.len() - 1,
        "GetComputerNameExW changed the buffer size unexpectedly"
    );

    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[0..end])
}

#[cfg(all(test, any(unix, windows), feature = "std"))]
mod tests {
    use std::process::Command;

    #[test]
    fn gethostname_matches_system_hostname() {
        let output = Command::new("hostname")
            .output()
            .expect("failed to get hostname");
        if output.status.success() {
            let hostname = String::from_utf8_lossy(&output.stdout);
            assert!(
                !hostname.is_empty(),
                "Failed to get hostname: hostname empty?"
            );
            // Convert both sides to lowercase; hostnames are case-insensitive
            // anyway.
            assert_eq!(
                super::gethostname().into_string().unwrap().to_lowercase(),
                hostname.trim_end().to_lowercase()
            );
        } else {
            panic!(
                "Failed to get hostname! {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[test]
    #[ignore]
    fn gethostname_matches_fixed_hostname() {
        assert_eq!(
            super::gethostname().into_string().unwrap().to_lowercase(),
            "hostname-for-testing"
        );
    }
}
