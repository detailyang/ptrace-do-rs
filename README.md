<p align="center">
  <b>
    <span style="font-size:larger;">ptrace-do-rs</span>
  </b>
  <br />
   <a href="https://travis-ci.org/detailyang/ptrace-do-rs"><img src="https://travis-ci.org/detailyang/ptrace-do-rs.svg?branch=master" /></a>
   <a href="https://ci.appveyor.com/project/detailyang/ptrace-do-rs"><img src="https://ci.appveyor.com/api/projects/status/drc2xk4kcoiydr0x?svg=true" /></a>
   <br />
   <b>ptrace-do-rs is rust bindings for libptrace-do</b>
</p>

# ptrace-do-rs

rust bindings to [libptrace_do](https://github.com/emptymonkey/ptrace_do) via [bindgen](https://github.com/rust-lang/rust-bindgen)

# requirements
* arch: linux/amd64
* llvm:  ^7.0
* clang: ^7.0
* bindgen: ^0.5

# artifact

## closeany

closeany is the binary to close process fd which attach the target process then close the specified fd via ptrace which can be used to kill the close-wait tcp connection.

> closeany is equal to the command: `gdb -ex="set confirm off" -p pid -ex 'p close(fd)' -ex quit`

### usage

```bash
closeany -p pid fd1 fd2 fd3
```

# License
ptrace-do-rs is under the [MIT license](/LICENSE). See the [LICENSE](/LICENSE) file for details.