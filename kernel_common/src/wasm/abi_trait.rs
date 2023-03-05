use super::abi::Context;

// See: https://docs.rs/wasmi_wasi/latest/src/wasmi_wasi/sync/snapshots/preview_1.rs.html#92-733
pub trait Abi: Send + Sync {
    fn args_get(&self, caller: Context, _argv: i32, _argv_buf: i32) -> i32 { todo!("args_get"); }
    fn args_sizes_get(&self, caller: Context, _offset0: i32, _offset1: i32) -> i32 { todo!("args_sizes_get"); }
    fn environ_get(&self, caller: Context, _environ: i32, _environ_buf: i32) -> i32 { todo!("environ_get"); }
    fn environ_sizes_get(&self, caller: Context, _offset0: i32, _offset1: i32) -> i32 { todo!("environ_sizes_get"); }
    fn clock_res_get(&self, caller: Context, _id: i32, _offset0: i32) -> i32 { todo!("clock_res_get"); }
    fn clock_time_get(&self, caller: Context, _id: i32, _precision: i64, _offset0: i32) -> i32 { todo!("clock_time_get"); }
    fn fd_advise(&self, caller: Context, _fd: i32, _offset: i64, _len: i64, _advice: i32) -> i32 { todo!("fd_advise"); }
    fn fd_allocate(&self, caller: Context, _fd: i32, _offset: i64, _len: i64) -> i32 { todo!("fd_allocate"); }
    fn fd_close(&self, caller: Context, _fd: i32) -> i32 { todo!("fd_close"); }
    fn fd_datasync(&self, caller: Context, _fd: i32) -> i32 { todo!("fd_datasync"); }
    fn fd_fdstat_get(&self, caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_fdstat_get"); }
    fn fd_fdstat_set_flags(&self, caller: Context, _fd: i32, _flags: i32) -> i32 { todo!("fd_fdstat_set_flags"); }
    fn fd_fdstat_set_rights(&self, caller: Context, _fd: i32, _fs_rights_base: i64, _fs_rights_inheriting: i64) -> i32 { todo!("fd_fdstat_set_rights"); }
    fn fd_filestat_get(&self, caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_filestat_get"); }
    fn fd_filestat_set_size(&self, caller: Context, _fd: i32, _size: i64) -> i32 { todo!("fd_filestat_set_size"); }
    fn fd_filestat_set_times(&self, caller: Context, _fd: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!("fd_filestat_set_times"); }
    fn fd_pread(&self, caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!("fd_pread"); }
    fn fd_prestat_get(&self, caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_prestat_get"); }
    fn fd_prestat_dir_name(&self, caller: Context, _fd: i32, _path: i32, _path_len: i32) -> i32 { todo!("fd_prestat_dir_name"); }
    fn fd_pwrite(&self, caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!("fd_pwrite"); }
    fn fd_read(&self, caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset1: i32) -> i32 { todo!("fd_read"); }
    fn fd_readdir(&self, caller: Context, _fd: i32, _buf: i32, _buf_len: i32, _cookie: i64, _offset0: i32) -> i32 { todo!("fd_readdir"); }
    fn fd_renumber(&self, caller: Context, _fd: i32, _to: i32) -> i32 { todo!("fd_renumber"); }
    fn fd_seek(&self, caller: Context, _fd: i32, _offset: i64, _whence: i32, _offset0: i32) -> i32 { todo!("fd_seek"); }
    fn fd_sync(&self, caller: Context, _fd: i32) -> i32 { todo!("fd_sync"); }
    fn fd_tell(&self, caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_tell"); }
    fn fd_write(&self, caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset0: i32) -> i32 { todo!("fd_write"); }
    fn path_create_directory(&self, caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_create_directory"); }
    fn path_filestat_get(&self, caller: Context, _fd: i32, _flags: i32, _offset: i32, _length: i32, _offset0: i32) -> i32 { todo!("path_filestat_get"); }
    fn path_filestat_set_times(&self, caller: Context, _fd: i32, _flags: i32, _offset: i32, _length: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!("path_filestat_set_times"); }
    fn path_link(&self, caller: Context, _old_fd: i32, _old_flags: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_link"); }
    fn path_open(&self, caller: Context, _fd: i32, _dirflags: i32, _offset: i32, _length: i32, _oflags: i32, _fs_rights_base: i64, _fdflags: i64, _fs_rights_inheriting: i32, _offfset0: i32) -> i32 { todo!("path_open"); }
    fn path_readlink(&self, caller: Context, _fd: i32, _offset: i32, _length: i32, _buf: i32, _buf_len: i32, _offset0: i32) -> i32 { todo!("path_readlink"); }
    fn path_remove_directory(&self, caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_remove_directory"); }
    fn path_rename(&self, caller: Context, _fd: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_rename"); }
    fn path_symlink(&self, caller: Context, _old_offset: i32, _old_length: i32, _fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_symlink"); }
    fn path_unlink_file(&self, caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_unlink_file"); }
    fn poll_oneoff(&self, caller: Context, _in_: i32, _out: i32, _nsubscriptions: i32, _offset0: i32) -> i32 { todo!("poll_oneoff"); }
    fn proc_exit(&self, caller: Context, _rval: i32) -> () { todo!("proc_exit"); }
    fn proc_raise(&self, caller: Context, _sig: i32) -> i32 { todo!("proc_raise"); }
    fn sched_yield(&self) -> i32 { todo!("sched_yield"); }
    fn random_get(&self, caller: Context, _buf: i32, _buf_len: i32) -> i32 { todo!("random_get"); }
    fn sock_accept(&self, caller: Context, _fd: i32, _flags: i32, _offset0: i32) -> i32 { todo!("sock_accept"); }
    fn sock_recv(&self, caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _ri_flags: i32, _offset0: i32, _offset1: i32) -> i32 { todo!("sock_recv"); }
    fn sock_send(&self, caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _si_flags: i32, _offset0: i32) -> i32 { todo!("sock_send"); }
    fn sock_shutdown(&self, caller: Context, _fd: i32, _how: i32) -> i32 { todo!("sock_shutdown"); }
}
