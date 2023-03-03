use core::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};
use alloc::boxed::Box;
use wasmi::Error;

// See: https://docs.rs/wasmi_wasi/latest/src/wasmi_wasi/sync/snapshots/preview_1.rs.html#92-733
pub trait Abi: Send + Sync {
    fn args_get(&self, argv: i32, argv_buf: i32) -> i32 { todo!(); }
    fn args_sizes_get(&self, offset0: i32, offset1: i32) -> i32 { todo!(); }
    fn environ_get(&self, environ: i32, environ_buf: i32) -> i32 { todo!(); }
    fn environ_sizes_get(&self, offset0: i32, offset1: i32) -> i32 { todo!(); }
    fn clock_res_get(&self, id: i32, offset0: i32) -> i32 { todo!(); }
    fn clock_time_get(&self, id: i32, precision: i64, offset0: i32) -> i32 { todo!(); }
    fn fd_advise(&self, fd: i32, offset: i64, len: i64, advice: i32) -> i32 { todo!(); }
    fn fd_allocate(&self, fd: i32, offset: i64, len: i64) -> i32 { todo!(); }
    fn fd_close(&self, fd: i32) -> i32 { todo!(); }
    fn fd_datasync(&self, fd: i32) -> i32 { todo!(); }
    fn fd_fdstat_get(&self, fd: i32, offset0: i32) -> i32 { todo!(); }
    fn fd_fdstat_set_flags(&self, fd: i32, flags: i32) -> i32 { todo!(); }
    fn fd_fdstat_set_rights(&self, fd: i32, fs_rights_base: i64, fs_rights_inheriting: i64) -> i32 { todo!(); }
    fn fd_filestat_get(&self, fd: i32, offset0: i32) -> i32 { todo!(); }
    fn fd_filestat_set_size(&self, fd: i32, size: i64) -> i32 { todo!(); }
    fn fd_filestat_set_times(&self, fd: i32, atim: i64, mtim: i64, fst_flags: i32) -> i32 { todo!(); }
    fn fd_pread(&self, fd: i32, iov_buf: i32, iov_buf_len: i32, offset: i64, offset0: i32) -> i32 { todo!(); }
    fn fd_prestat_get(&self, fd: i32, offset0: i32) -> i32 { todo!(); }
    fn fd_prestat_dir_name(&self, fd: i32, path: i32, path_len: i32) -> i32 { todo!(); }
    fn fd_pwrite(&self, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset: i64, offset0: i32) -> i32 { todo!(); }
    fn fd_read(&self, fd: i32, iov_buf: i32, iov_buf_len: i32, offset1: i32) -> i32 { todo!(); }
    fn fd_readdir(&self, fd: i32, buf: i32, buf_len: i32, cookie: i64, offset0: i32) -> i32 { todo!(); }
    fn fd_renumber(&self, fd: i32, to: i32) -> i32 { todo!(); }
    fn fd_seek(&self, fd: i32, offset: i64, whence: i32, offset0: i32) -> i32 { todo!(); }
    fn fd_sync(&self, fd: i32) -> i32 { todo!(); }
    fn fd_tell(&self, fd: i32, offset0: i32) -> i32 { todo!(); }
    fn fd_write(&self, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32) -> i32 { todo!(); }
    fn path_create_directory(&self, fd: i32, offset: i32, length: i32) -> i32 { todo!(); }
    fn path_filestat_get(&self, fd: i32, flags: i32, offset: i32, length: i32, offset0: i32) -> i32 { todo!(); }
    fn path_filestat_set_times(&self, fd: i32, flags: i32, offset: i32, length: i32, atim: i64, mtim: i64, fst_flags: i32) -> i32 { todo!(); }
    fn path_link(&self, old_fd: i32, old_flags: i32, old_offset: i32, old_length: i32, new_fd: i32, new_offset: i32, new_length: i32) -> i32 { todo!(); }
    fn path_open(&self, fd: i32, dirflags: i32, offset: i32, length: i32, oflags: i32, fs_rights_base: i64, fdflags: i64, fs_rights_inheriting: i32, offfset0: i32) -> i32 { todo!(); }
    fn path_readlink(&self, fd: i32, offset: i32, length: i32, buf: i32, buf_len: i32, offset0: i32) -> i32 { todo!(); }
    fn path_remove_directory(&self, fd: i32, offset: i32, length: i32) -> i32 { todo!(); }
    fn path_rename(&self, fd: i32, old_offset: i32, old_length: i32, new_fd: i32, new_offset: i32, new_length: i32) -> i32 { todo!(); }
    fn path_symlink(&self, old_offset: i32, old_length: i32, fd: i32, new_offset: i32, new_length: i32) -> i32 { todo!(); }
    fn path_unlink_file(&self, fd: i32, offset: i32, length: i32) -> i32 { todo!(); }
    fn poll_oneoff(&self, in_: i32, out: i32, nsubscriptions: i32, offset0: i32) -> i32 { todo!(); }
    fn proc_exit(&self, rval: i32) -> () { todo!(); }
    fn proc_raise(&self, sig: i32) -> i32 { todo!(); }
    fn sched_yield(&self) -> i32 { todo!(); }
    fn random_get(&self, buf: i32, buf_len: i32) -> i32 { todo!(); }
    fn sock_accept(&self, fd: i32, flags: i32, offset0: i32) -> i32 { todo!(); }
    fn sock_recv(&self, fd: i32, iov_buf: i32, iov_buf_len: i32, ri_flags: i32, offset0: i32, offset1: i32) -> i32 { todo!(); }
    fn sock_send(&self, fd: i32, ciov_buf: i32, ciov_buf_len: i32, si_flags: i32, offset0: i32) -> i32 { todo!(); }
    fn sock_shutdown(&self, fd: i32, how: i32) -> i32 { todo!(); }
}
