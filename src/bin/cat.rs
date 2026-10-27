use std::{
    env,
    ffi::{CStr, CString, OsStr},
    io,
    os::{fd::RawFd, unix::ffi::OsStrExt},
};

fn open(path: &CStr) -> io::Result<RawFd> {
    let fd = unsafe { libc::open(path.as_ptr(), libc::O_RDONLY) };
    if fd == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(fd)
    }
}

fn read(fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
    loop {
        let n = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };

        if n >= 0 {
            return Ok(n as usize);
        }

        let err = io::Error::last_os_error();
        if err.kind() != io::ErrorKind::Interrupted {
            return Err(err);
        }
    }
}

fn write_all(fd: RawFd, mut buf: &[u8]) -> io::Result<()> {
    while !buf.is_empty() {
        let n = unsafe { libc::write(fd, buf.as_ptr().cast(), buf.len()) };

        if n < 0 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(err);
        }

        if n == 0 {
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }

        buf = &buf[n as usize..];
    }

    Ok(())
}

fn cat(fd: RawFd) -> io::Result<()> {
    let mut buf = [0u8; 8192];

    loop {
        let n = read(fd, &mut buf)?;

        if n == 0 {
            break;
        }

        write_all(libc::STDOUT_FILENO, &buf[..n])?;
    }

    Ok(())
}

fn cat_file(path: &OsStr) -> io::Result<()> {
    if path.as_bytes() == b"-" {
        return cat(libc::STDIN_FILENO);
    }

    let path = CString::new(path.as_bytes())
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    let fd = open(&path)?;
    let result = cat(fd);

    let close_result = unsafe { libc::close(fd) };
    if close_result == -1 && result.is_ok() {
        return Err(io::Error::last_os_error());
    }

    result
}

fn main() -> io::Result<()> {
    let mut args = env::args_os().skip(1);

    if let Some(path) = args.next() {
        cat_file(&path)?;

        for path in args {
            cat_file(&path)?;
        }
    } else {
        cat(libc::STDIN_FILENO)?;
    }

    Ok(())
}
