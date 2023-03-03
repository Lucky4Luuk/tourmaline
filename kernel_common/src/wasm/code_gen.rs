vec![
	AbiFunc::wrap("args_get", store, |mut caller: Caller<'_, ()>, argv: i32, argv_buf: i32| self.args_get(argv, argv_buf)),
	AbiFunc::wrap("args_sizes_get", store, |mut caller: Caller<'_, ()>, offset0: i32, offset1: i32| self.args_sizes_get(offset0, offset1)),
	AbiFunc::wrap("environ_get", store, |mut caller: Caller<'_, ()>, environ: i32, environ_buf: i32| self.environ_get(environ, environ_buf)),
	AbiFunc::wrap("environ_sizes_get", store, |mut caller: Caller<'_, ()>, offset0: i32, offset1: i32| self.environ_sizes_get(offset0, offset1)),
	AbiFunc::wrap("clock_res_get", store, |mut caller: Caller<'_, ()>, id: i32, offset0: i32| self.clock_res_get(id, offset0)),
	AbiFunc::wrap("clock_time_get", store, |mut caller: Caller<'_, ()>, id: i32, precision: i64, offset0: i32| self.clock_time_get(id, precision, offset0)),
	AbiFunc::wrap("fd_advise", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i64, len: i64, advice: i32| self.fd_advise(fd, offset, len, advice)),
	AbiFunc::wrap("fd_allocate", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i64, len: i64| self.fd_allocate(fd, offset, len)),
	AbiFunc::wrap("fd_close", store, |mut caller: Caller<'_, ()>, fd: i32| self.fd_close(fd)),
	AbiFunc::wrap("fd_datasync", store, |mut caller: Caller<'_, ()>, fd: i32| self.fd_datasync(fd)),
	AbiFunc::wrap("fd_fdstat_get", store, |mut caller: Caller<'_, ()>, fd: i32, offset0: i32| self.fd_fdstat_get(fd, offset0)),
	AbiFunc::wrap("fd_fdstat_set_flags", store, |mut caller: Caller<'_, ()>, fd: i32, flags: i32| self.fd_fdstat_set_flags(fd, flags)),
	AbiFunc::wrap("fd_fdstat_set_rights", store, |mut caller: Caller<'_, ()>, fd: i32, fs_rights_base: i64, fs_rights_inheriting: i64| self.fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting)),
	AbiFunc::wrap("fd_filestat_get", store, |mut caller: Caller<'_, ()>, fd: i32, offset0: i32| self.fd_filestat_get(fd, offset0)),
	AbiFunc::wrap("fd_filestat_set_size", store, |mut caller: Caller<'_, ()>, fd: i32, size: i64| self.fd_filestat_set_size(fd, size)),
	AbiFunc::wrap("fd_filestat_set_times", store, |mut caller: Caller<'_, ()>, fd: i32, atim: i64, mtim: i64, fst_flags: i32| self.fd_filestat_set_times(fd, atim, mtim, fst_flags)),
	AbiFunc::wrap("fd_pread", store, |mut caller: Caller<'_, ()>, fd: i32, iov_buf: i32, iov_buf_len: i32, offset: i64, offset0: i32| self.fd_pread(fd, iov_buf, iov_buf_len, offset, offset0)),
	AbiFunc::wrap("fd_prestat_get", store, |mut caller: Caller<'_, ()>, fd: i32, offset0: i32| self.fd_prestat_get(fd, offset0)),
	AbiFunc::wrap("fd_prestat_dir_name", store, |mut caller: Caller<'_, ()>, fd: i32, path: i32, path_len: i32| self.fd_prestat_dir_name(fd, path, path_len)),
	AbiFunc::wrap("fd_pwrite", store, |mut caller: Caller<'_, ()>, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset: i64, offset0: i32| self.fd_pwrite(fd, ciov_buf, ciov_buf_len, offset, offset0)),
	AbiFunc::wrap("fd_read", store, |mut caller: Caller<'_, ()>, fd: i32, iov_buf: i32, iov_buf_len: i32, offset1: i32| self.fd_read(fd, iov_buf, iov_buf_len, offset1)),
	AbiFunc::wrap("fd_readdir", store, |mut caller: Caller<'_, ()>, fd: i32, buf: i32, buf_len: i32, cookie: i64, offset0: i32| self.fd_readdir(fd, buf, buf_len, cookie, offset0)),
	AbiFunc::wrap("fd_renumber", store, |mut caller: Caller<'_, ()>, fd: i32, to: i32| self.fd_renumber(fd, to)),
	AbiFunc::wrap("fd_seek", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i64, whence: i32, offset0: i32| self.fd_seek(fd, offset, whence, offset0)),
	AbiFunc::wrap("fd_sync", store, |mut caller: Caller<'_, ()>, fd: i32| self.fd_sync(fd)),
	AbiFunc::wrap("fd_tell", store, |mut caller: Caller<'_, ()>, fd: i32, offset0: i32| self.fd_tell(fd, offset0)),
	AbiFunc::wrap("fd_write", store, |mut caller: Caller<'_, ()>, fd: i32, ciov_buf: i32, ciov_buf_len: i32, offset0: i32| self.fd_write(fd, ciov_buf, ciov_buf_len, offset0)),
	AbiFunc::wrap("path_create_directory", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i32, length: i32| self.path_create_directory(fd, offset, length)),
	AbiFunc::wrap("path_filestat_get", store, |mut caller: Caller<'_, ()>, fd: i32, flags: i32, offset: i32, length: i32, offset0: i32| self.path_filestat_get(fd, flags, offset, length, offset0)),
	AbiFunc::wrap("path_filestat_set_times", store, |mut caller: Caller<'_, ()>, fd: i32, flags: i32, offset: i32, length: i32, atim: i64, mtim: i64, fst_flags: i32| self.path_filestat_set_times(fd, flags, offset, length, atim, mtim, fst_flags)),
	AbiFunc::wrap("path_link", store, |mut caller: Caller<'_, ()>, old_fd: i32, old_flags: i32, old_offset: i32, old_length: i32, new_fd: i32, new_offset: i32, new_length: i32| self.path_link(old_fd, old_flags, old_offset, old_length, new_fd, new_offset, new_length)),
	AbiFunc::wrap("path_open", store, |mut caller: Caller<'_, ()>, fd: i32, dirflags: i32, offset: i32, length: i32, oflags: i32, fs_rights_base: i64, fdflags: i64, fs_rights_inheriting: i32, offfset0: i32| self.path_open(fd, dirflags, offset, length, oflags, fs_rights_base, fdflags, fs_rights_inheriting, offfset0)),
	AbiFunc::wrap("path_readlink", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i32, length: i32, buf: i32, buf_len: i32, offset0: i32| self.path_readlink(fd, offset, length, buf, buf_len, offset0)),
	AbiFunc::wrap("path_remove_directory", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i32, length: i32| self.path_remove_directory(fd, offset, length)),
	AbiFunc::wrap("path_rename", store, |mut caller: Caller<'_, ()>, fd: i32, old_offset: i32, old_length: i32, new_fd: i32, new_offset: i32, new_length: i32| self.path_rename(fd, old_offset, old_length, new_fd, new_offset, new_length)),
	AbiFunc::wrap("path_symlink", store, |mut caller: Caller<'_, ()>, old_offset: i32, old_length: i32, fd: i32, new_offset: i32, new_length: i32| self.path_symlink(old_offset, old_length, fd, new_offset, new_length)),
	AbiFunc::wrap("path_unlink_file", store, |mut caller: Caller<'_, ()>, fd: i32, offset: i32, length: i32| self.path_unlink_file(fd, offset, length)),
	AbiFunc::wrap("poll_oneoff", store, |mut caller: Caller<'_, ()>, in_: i32, out: i32, nsubscriptions: i32, offset0: i32| self.poll_oneoff(in_, out, nsubscriptions, offset0)),
	AbiFunc::wrap("proc_exit", store, |mut caller: Caller<'_, ()>, rval: i32| self.proc_exit(rval)),
	AbiFunc::wrap("proc_raise", store, |mut caller: Caller<'_, ()>, sig: i32| self.proc_raise(sig)),
	AbiFunc::wrap("sched_yield", store, |mut caller: Caller<'_, ()>| self.sched_yield()),
	AbiFunc::wrap("random_get", store, |mut caller: Caller<'_, ()>, buf: i32, buf_len: i32| self.random_get(buf, buf_len)),
	AbiFunc::wrap("sock_accept", store, |mut caller: Caller<'_, ()>, fd: i32, flags: i32, offset0: i32| self.sock_accept(fd, flags, offset0)),
	AbiFunc::wrap("sock_recv", store, |mut caller: Caller<'_, ()>, fd: i32, iov_buf: i32, iov_buf_len: i32, ri_flags: i32, offset0: i32, offset1: i32| self.sock_recv(fd, iov_buf, iov_buf_len, ri_flags, offset0, offset1)),
	AbiFunc::wrap("sock_send", store, |mut caller: Caller<'_, ()>, fd: i32, ciov_buf: i32, ciov_buf_len: i32, si_flags: i32, offset0: i32| self.sock_send(fd, ciov_buf, ciov_buf_len, si_flags, offset0)),
	AbiFunc::wrap("sock_shutdown", store, |mut caller: Caller<'_, ()>, fd: i32, how: i32| self.sock_shutdown(fd, how)),
]