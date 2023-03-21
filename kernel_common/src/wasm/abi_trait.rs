use super::abi::{Context, yield_now};

/// Defines the required implementations to implement the full
/// tourmaline ABI. This includes the wasi ABI and various custom
/// designed ABIs.
pub trait Abi: Send + Sync {
    // See: https://docs.rs/wasmi_wasi/latest/src/wasmi_wasi/sync/snapshots/preview_1.rs.html#92-733
    // ENV: wasi_snapshot_preview1
    fn args_get(&self, _caller: Context, _argv: i32, _argv_buf: i32) -> i32 { todo!("args_get"); }
    fn args_sizes_get(&self, _caller: Context, _offset0: i32, _offset1: i32) -> i32 { todo!("args_sizes_get"); }
    fn environ_get(&self, _caller: Context, _environ: i32, _environ_buf: i32) -> i32 { todo!("environ_get"); }
    fn environ_sizes_get(&self, _caller: Context, _offset0: i32, _offset1: i32) -> i32 { todo!("environ_sizes_get"); }
    fn clock_res_get(&self, _caller: Context, _id: i32, _offset0: i32) -> i32 { todo!("clock_res_get"); }
    fn clock_time_get(&self, _caller: Context, _id: i32, _precision: i64, _offset0: i32) -> i32 { todo!("clock_time_get"); }
    fn fd_advise(&self, _caller: Context, _fd: i32, _offset: i64, _len: i64, _advice: i32) -> i32 { todo!("fd_advise"); }
    fn fd_allocate(&self, _caller: Context, _fd: i32, _offset: i64, _len: i64) -> i32 { todo!("fd_allocate"); }
    fn fd_close(&self, _caller: Context, _fd: i32) -> i32 { todo!("fd_close"); }
    fn fd_datasync(&self, _caller: Context, _fd: i32) -> i32 { todo!("fd_datasync"); }
    fn fd_fdstat_get(&self, _caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_fdstat_get"); }
    fn fd_fdstat_set_flags(&self, _caller: Context, _fd: i32, _flags: i32) -> i32 { todo!("fd_fdstat_set_flags"); }
    fn fd_fdstat_set_rights(&self, _caller: Context, _fd: i32, _fs_rights_base: i64, _fs_rights_inheriting: i64) -> i32 { todo!("fd_fdstat_set_rights"); }
    fn fd_filestat_get(&self, _caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_filestat_get"); }
    fn fd_filestat_set_size(&self, _caller: Context, _fd: i32, _size: i64) -> i32 { todo!("fd_filestat_set_size"); }
    fn fd_filestat_set_times(&self, _caller: Context, _fd: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!("fd_filestat_set_times"); }
    fn fd_pread(&self, _caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!("fd_pread"); }
    fn fd_prestat_get(&self, _caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_prestat_get"); }
    fn fd_prestat_dir_name(&self, _caller: Context, _fd: i32, _path: i32, _path_len: i32) -> i32 { todo!("fd_prestat_dir_name"); }
    fn fd_pwrite(&self, _caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!("fd_pwrite"); }
    fn fd_read(&self, _caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset1: i32) -> i32 { todo!("fd_read"); }
    fn fd_readdir(&self, _caller: Context, _fd: i32, _buf: i32, _buf_len: i32, _cookie: i64, _offset0: i32) -> i32 { todo!("fd_readdir"); }
    fn fd_renumber(&self, _caller: Context, _fd: i32, _to: i32) -> i32 { todo!("fd_renumber"); }
    fn fd_seek(&self, _caller: Context, _fd: i32, _offset: i64, _whence: i32, _offset0: i32) -> i32 { todo!("fd_seek"); }
    fn fd_sync(&self, _caller: Context, _fd: i32) -> i32 { todo!("fd_sync"); }
    fn fd_tell(&self, _caller: Context, _fd: i32, _offset0: i32) -> i32 { todo!("fd_tell"); }
    fn fd_write(&self, _caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset0: i32) -> i32 { todo!("fd_write"); }
    fn path_create_directory(&self, _caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_create_directory"); }
    fn path_filestat_get(&self, _caller: Context, _fd: i32, _flags: i32, _offset: i32, _length: i32, _offset0: i32) -> i32 { todo!("path_filestat_get"); }
    fn path_filestat_set_times(&self, _caller: Context, _fd: i32, _flags: i32, _offset: i32, _length: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!("path_filestat_set_times"); }
    fn path_link(&self, _caller: Context, _old_fd: i32, _old_flags: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_link"); }
    fn path_open(&self, _caller: Context, _fd: i32, _dirflags: i32, _offset: i32, _length: i32, _oflags: i32, _fs_rights_base: i64, _fdflags: i64, _fs_rights_inheriting: i32, _offfset0: i32) -> i32 { todo!("path_open"); }
    fn path_readlink(&self, _caller: Context, _fd: i32, _offset: i32, _length: i32, _buf: i32, _buf_len: i32, _offset0: i32) -> i32 { todo!("path_readlink"); }
    fn path_remove_directory(&self, _caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_remove_directory"); }
    fn path_rename(&self, _caller: Context, _fd: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_rename"); }
    fn path_symlink(&self, _caller: Context, _old_offset: i32, _old_length: i32, _fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!("path_symlink"); }
    fn path_unlink_file(&self, _caller: Context, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!("path_unlink_file"); }
    fn poll_oneoff(&self, _caller: Context, _in_: i32, _out: i32, _nsubscriptions: i32, _offset0: i32) -> i32 { todo!("poll_oneoff"); }
    fn proc_exit(&self, _caller: Context, _rval: i32) -> () { todo!("proc_exit"); }
    fn proc_raise(&self, _caller: Context, _sig: i32) -> i32 { todo!("proc_raise"); }
    /// Default implementation just yields the program. Only replace if you know what you are doing!
    fn sched_yield(&self) -> Result<(), wasmi::core::Trap> { yield_now() }
    fn random_get(&self, _caller: Context, _buf: i32, _buf_len: i32) -> i32 { todo!("random_get"); }
    fn sock_accept(&self, _caller: Context, _fd: i32, _flags: i32, _offset0: i32) -> i32 { todo!("sock_accept"); }
    fn sock_recv(&self, _caller: Context, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _ri_flags: i32, _offset0: i32, _offset1: i32) -> i32 { todo!("sock_recv"); }
    fn sock_send(&self, _caller: Context, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _si_flags: i32, _offset0: i32) -> i32 { todo!("sock_send"); }
    fn sock_shutdown(&self, _caller: Context, _fd: i32, _how: i32) -> i32 { todo!("sock_shutdown"); }

    // ENV: sys_abi
    fn yield_now(&self) -> Result<(), wasmi::core::Trap> { yield_now() }
    fn poll_promise(&self, mut caller: Context, promise_id: i32) -> i32 { caller.poll_promise(promise_id) }

    // ENV: driver_abi
    fn driver_write(&self, _caller: Context, _name_ptr: i32, _name_len: i32, _cmd: i32, _data_ptr: i32, _data_len: i32) -> i32 { todo!("call_driver"); }
    fn driver_read(&self, _caller: Context, _name_ptr: i32, _name_len: i32, _cmd: i32, _data_ptr: i32, _data_len: i32) -> i32 { todo!("call_driver"); }

    // ENV: host_abi
    fn host_memset(&self, _caller: Context, _addr: i32, _data_ptr: i32, _data_len: i32) -> i32 { todo!("host_memset"); }
    fn host_memread(&self, _caller: Context, _read_addr: i32, _buf_ptr: i32, _buf_len: i32) -> i32 { todo!("host_memread"); }
}
