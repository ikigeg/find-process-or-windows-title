# find-process-or-windows-title

Does exactly what it says on the tin, simply offering a way to search for any windows with a matching title, or a process with matching name.

To search for a process by partial name:

`find-process-or-windows-title.exe -pn "partial name"`

To search for a window title containing text:

`find-process-or-windows-title.exe -wt "containing text"`

And you can also search with both:

`find-process-or-windows-title.exe -pn "partial name" -wt "containing text"`

With any matches you'll get a list with pattern:

```
Title: Default IME, Process: RAVCpl64.exe, PID: 9856, PPID: 4412
Title: Default IME, Process: TextInputHost.exe, PID: 10224, PPID: 1364
Title: Default IME, Process: PhoneExperienceHost.exe, PID: 3340, PPID: 1364
Title: Default IME, Process: RuntimeBroker.exe, PID: 2968, PPID: 1364
Title: Default IME, Process: SearchHost.exe, PID: 7960, PPID: 1364
Title: Default IME, Process: Explorer.EXE, PID: 4412, PPID: 3980
Title: Default IME, Process: Explorer.EXE, PID: 4412, PPID: 3980
Title: Default IME, Process: svchost.exe, PID: 1568, PPID: 1140
Title: Default IME, Process: Explorer.EXE, PID: 4412, PPID: 3980
Title: Default IME, Process: Explorer.EXE, PID: 4412, PPID: 3980
Title: MSCTFIME UI, Process: Explorer.EXE, PID: 4412, PPID: 3980
Title: MSCTFIME UI, Process: RustDesk.exe, PID: 6652, PPID: 5612
Title: Default IME, Process: RustDesk.exe, PID: 6652, PPID: 5612
```

When there are no matches you will see:

`No matches!`

If you simply want to list everything out, simply run with no arguments:

`find-process-or-windows-title.exe`

## To build this project

First install Rust `https://www.rust-lang.org/tools/install`

To run the project `cargo run`

To run with arguments `cargo run -- -pn "partial name" -wt "containing text"`

To build this project on a windows machine `cargo build --release`

To build this for Windows from a mac:

```
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```
