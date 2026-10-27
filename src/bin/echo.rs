use std::io;
use std::os::unix::ffi::OsStrExt;

fn write_stdout(buf: &[u8]) -> io::Result<()> {
    let mut written = 0;

    while written < buf.len() {
        let remaining = &buf[written..];

        let n = unsafe {
            libc::write(
                libc::STDOUT_FILENO,
                remaining.as_ptr().cast(),
                remaining.len(),
            )
        };

        if n < 0 {
            let err = io::Error::last_os_error();

            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }

            return Err(err);
        }

        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "write returned zero",
            ));
        }

        written += n as usize;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args_os().skip(1).peekable();

    let newline = if args.peek().is_some_and(|s| s == "-n") {
        args.next();
        false
    } else {
        true
    };

    let mut first = true;

    for arg in args {
        if !first {
            write_stdout(b" ")?;
        }
        write_stdout(arg.as_bytes())?;
        first = false
    }

    if newline {
        write_stdout(b"\n")?;
    }

    Ok(())
}
