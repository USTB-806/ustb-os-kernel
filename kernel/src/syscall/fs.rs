//! File and filesystem-related syscalls

use log::trace;
use crate::print;
use crate::utils::sbi::console_getchar;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;
const FD_MOCK_FILE: usize = 3;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn sys_read(fd: usize, buf: *mut u8, _len: usize) -> isize {
    trace!("kernel: sys_read");
    match fd {
        FD_STDIN => { // 从用户空间内存读入 1 个字符
            let c = console_getchar();
            if c == 0 { return 0; }
            unsafe { *buf = c as u8; }
            1
        }
        FD_MOCK_FILE => {
            let _msg = b"Success!\n";
            // 需要将内核数据 msg 复制到用户空间缓冲区 buf 中
            // 关于 core::ptr::copy 参见 https://doc.rust-lang.org/core/ptr/fn.copy.html
            // 函数返回值为你实际读取的字节数 copy_len，copy_len 为 syscall 的参数缓冲区长度上限 len 和你读取字节长度的最小值
            0
        }
        _ => -1,
    }
}

pub fn sys_open(_path: *const u8, _flags: u32) -> isize {
    trace!("kernel: sys_open");
    // 需要让 s 的值为当前打开文件的文件名，调用此 syscall 将会输出 sys_open: path = test_file
    // 为此需要先对 path 使用 add 方法，得到 path 到文件名尾部的偏移量长度
    // 关于 add method 参见 https://doc.rust-lang.org/core/primitive.pointer.html#method.add
    // 然后使用 from_raw_parts 和 from_utf8 得到文件名 s，并输出
    // 关于 from_raw_parts 参见 https://doc.rust-lang.org/core/slice/fn.from_raw_parts.html
    // 关于 from_utf8 参见 https://doc.rust-lang.org/core/str/fn.from_utf8.html
    // 一个自然的疑惑是为甚么需要指针运算，因为原始指针并不支持数组索引
    // 回顾在程序设计基础上的知识，数组索引 path[l] 实际上等价于指针运算 *path.add(l)

    let s = "";
    print!("sys_open: path = {}\n", s);
    
    FD_MOCK_FILE as isize
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel: sys_close");
    print!("sys_close: fd = {}\n", fd);
    0
}