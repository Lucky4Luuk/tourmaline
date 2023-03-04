// See: https://docs.rs/wasmi_wasi/latest/src/wasmi_wasi/sync/snapshots/preview_1.rs.html#92-733
pub trait Abi: Send + Sync {
    fn args_get(&self, _argv: i32, _argv_buf: i32) -> i32 { todo!(); }
    fn args_sizes_get(&self, _offset0: i32, _offset1: i32) -> i32 { todo!(); }
    fn environ_get(&self, _environ: i32, _environ_buf: i32) -> i32 { todo!(); }
    fn environ_sizes_get(&self, _offset0: i32, _offset1: i32) -> i32 { todo!(); }
    fn clock_res_get(&self, _id: i32, _offset0: i32) -> i32 { todo!(); }
    fn clock_time_get(&self, _id: i32, _precision: i64, _offset0: i32) -> i32 { todo!(); }
    fn fd_advise(&self, _fd: i32, _offset: i64, _len: i64, _advice: i32) -> i32 { todo!(); }
    fn fd_allocate(&self, _fd: i32, _offset: i64, _len: i64) -> i32 { todo!(); }
    fn fd_close(&self, _fd: i32) -> i32 { todo!(); }
    fn fd_datasync(&self, _fd: i32) -> i32 { todo!(); }
    fn fd_fdstat_get(&self, _fd: i32, _offset0: i32) -> i32 { todo!(); }
    fn fd_fdstat_set_flags(&self, _fd: i32, _flags: i32) -> i32 { todo!(); }
    fn fd_fdstat_set_rights(&self, _fd: i32, _fs_rights_base: i64, _fs_rights_inheriting: i64) -> i32 { todo!(); }
    fn fd_filestat_get(&self, _fd: i32, _offset0: i32) -> i32 { todo!(); }
    fn fd_filestat_set_size(&self, _fd: i32, _size: i64) -> i32 { todo!(); }
    fn fd_filestat_set_times(&self, _fd: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!(); }
    fn fd_pread(&self, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!(); }
    fn fd_prestat_get(&self, _fd: i32, _offset0: i32) -> i32 { todo!(); }
    fn fd_prestat_dir_name(&self, _fd: i32, _path: i32, _path_len: i32) -> i32 { todo!(); }
    fn fd_pwrite(&self, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset: i64, _offset0: i32) -> i32 { todo!(); }
    fn fd_read(&self, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _offset1: i32) -> i32 { todo!(); }
    fn fd_readdir(&self, _fd: i32, _buf: i32, _buf_len: i32, _cookie: i64, _offset0: i32) -> i32 { todo!(); }
    fn fd_renumber(&self, _fd: i32, _to: i32) -> i32 { todo!(); }
    fn fd_seek(&self, _fd: i32, _offset: i64, _whence: i32, _offset0: i32) -> i32 { todo!(); }
    fn fd_sync(&self, _fd: i32) -> i32 { todo!(); }
    fn fd_tell(&self, _fd: i32, _offset0: i32) -> i32 { todo!(); }
    fn fd_write(&self, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _offset0: i32) -> i32 { todo!(); }
    fn path_create_directory(&self, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!(); }
    fn path_filestat_get(&self, _fd: i32, _flags: i32, _offset: i32, _length: i32, _offset0: i32) -> i32 { todo!(); }
    fn path_filestat_set_times(&self, _fd: i32, _flags: i32, _offset: i32, _length: i32, _atim: i64, _mtim: i64, _fst_flags: i32) -> i32 { todo!(); }
    fn path_link(&self, _old_fd: i32, _old_flags: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!(); }
    fn path_open(&self, _fd: i32, _dirflags: i32, _offset: i32, _length: i32, _oflags: i32, _fs_rights_base: i64, _fdflags: i64, _fs_rights_inheriting: i32, _offfset0: i32) -> i32 { todo!(); }
    fn path_readlink(&self, _fd: i32, _offset: i32, _length: i32, _buf: i32, _buf_len: i32, _offset0: i32) -> i32 { todo!(); }
    fn path_remove_directory(&self, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!(); }
    fn path_rename(&self, _fd: i32, _old_offset: i32, _old_length: i32, _new_fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!(); }
    fn path_symlink(&self, _old_offset: i32, _old_length: i32, _fd: i32, _new_offset: i32, _new_length: i32) -> i32 { todo!(); }
    fn path_unlink_file(&self, _fd: i32, _offset: i32, _length: i32) -> i32 { todo!(); }
    fn poll_oneoff(&self, _in_: i32, _out: i32, _nsubscriptions: i32, _offset0: i32) -> i32 { todo!(); }
    fn proc_exit(&self, _rval: i32) -> () { todo!(); }
    fn proc_raise(&self, _sig: i32) -> i32 { todo!(); }
    fn sched_yield(&self) -> i32 { todo!(); }
    fn random_get(&self, _buf: i32, _buf_len: i32) -> i32 { todo!(); }
    fn sock_accept(&self, _fd: i32, _flags: i32, _offset0: i32) -> i32 { todo!(); }
    fn sock_recv(&self, _fd: i32, _iov_buf: i32, _iov_buf_len: i32, _ri_flags: i32, _offset0: i32, _offset1: i32) -> i32 { todo!(); }
    fn sock_send(&self, _fd: i32, _ciov_buf: i32, _ciov_buf_len: i32, _si_flags: i32, _offset0: i32) -> i32 { todo!(); }
    fn sock_shutdown(&self, _fd: i32, _how: i32) -> i32 { todo!(); }
}
