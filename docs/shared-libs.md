# list shared libraries used by a executables in Linux

Most of our programs load shared libraries at runtime, sometimes it's useful when debugging a unknown binary to list the shared libraries need by that program. So let's see here some ways to do that.

## Using *ldd*:

*ldd* is a shell script utility. it can output the shared libraries required by a binary:

	$ ldd /usr/bin/cat
		linux-vdso.so.1 (0x00007ffcba506000)
		libc.so.6 => /lib64/libc.so.6 (0x00007fb0faa0d000)
		/lib64/ld-linux-x86-64.so.2 (0x00007fb0fac39000)

The 'problem' with ldd is that it may execute the program to get the list of the shared libraries. So in a untrusted binary *ldd* may not be a safe choice.

## Using *objdump*:

*objdump* is very useful program that you may be familiar if you have some experience with debugging binaries. it's part of the GNU Binutils and it can also give us a list of shared libraries:

	$ objdump -p /usr/bin/cat | grep 'NEEDED'
	  NEEDED               libc.so.6

if we compare the output we can see that *ldd* lists more libraries than *objdump*. This is because the objdump command is dumping what the program itself lists as libraries. However, the ldd utility is listing which libraries ld.so would load. It follows the graph so that we can see what would be loaded by those libraries at runtime.

## Using *readelf*:

*readelf* it's another useful GNU Binutils' program for debugging and it can also give us a list of shared libraries:

	$ readelf --dynamic /usr/bin/cat | grep NEEDED
	 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]

## Using procfs if the program it's running

the procfs in Linux can output a lot of information about running programs. at `/proc/<pid>/maps` file we can find a description of a ragion of contiguous virtual memory in a process or thread. f the process has loaded a shared library, the library will show up in this file. As this file can ouput a lot of stuff you can filter only the shared libraries using:

	$ awk '$NF!~/\.so/{next} {$0=$NF} !a[$0]++' /proc/<pid>/maps

