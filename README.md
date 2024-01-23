<div align="center">
<h1>Rustils :hammer_and_wrench:</h1>
<p>Linux coreutils implemented in Rust. The purpose of this project is to develop a minimal, performant and consistent set of utilites for Linux systems</p>
<a href='#'><img src="https://img.shields.io/badge/Made%20with-Rust-&?style=flat-square&labelColor=232329&color=f74c00&logo=rust"/></a>
<a href='#'><img src="https://img.shields.io/badge/Maintained%3F-Yes-green.svg?style=flat-square&labelColor=232329&color=5277C3"></img></a>
<br/>

<a href="https://discord.gg/W4mQqNnfSq">
<img src="https://discordapp.com/api/guilds/913584348937207839/widget.png?style=shield"/></a>
</div>

## Implemented
<details open markdown="1"><summary><h3>File System</h3></summary>
  
- [ ] `chcon` - change file security context
- [ ] `chgrp` - change file group ownership
- [ ] `chown` - change file ownership
- [ ] `chmod` - change file/directory permissions
- [x] `cp` - copy file(s)/director[y|ies]
- [ ] `dd` - copy & convert file
- [ ] `df` - show free disk space  
- [ ] ~~`dir` - list files in director[y|(ies)] (`ls -Cb` equivalent)~~
- [ ] ~~`dircolors` - set up color for `ls`~~
- [ ] `install` - copy file(s) & set attributes
- [x] `ln` - make hard/symbolic links
- [x] `ls` - list files in director[y|(ies)]
- [x] `mkdir` - make director[y|(ies)]
- [x] `mv` - move file(s)/director[y|ies]
- [ ] `realpath` - returns the resolved absolute or relative path for a file
- [x] `rm` - remove file(s)
- [ ] `rmdir` - remove empty director[y|ies]
- [ ] `shred` - hide(/delete) file content
- [ ] `sync` - flush file system buffer
- [x] `touch` - make file(s)
- [ ] `truncate` - shrink/extend the size of a file
- [ ] ~~`vdir` - list files in director[y|(ies)] (`ls -l -b` equivalent)~~

</details>

---

<details open markdown="1"><summary><h3>Text Utilities</h3></summary>

- [ ] `b2sum` - computes and checks BLAKE2b message digest
- [ ] `base32` - encode/decode Base32; print to stdout
- [ ] `base64` - encode/decode Base64; print to stdout
- [x] `cat` - concatenate file(s) to stdout
- [ ] `cksum` - checksums and count bytes of a file
- [ ] `comm` - compare two sorted files line by line
- [ ] `csplit` - splits a file into sections determines by context lines
- [ ] `cut` - remove sections from each line of file(s)
- [ ] `expand` - convert tabs to spaces
- [ ] `fmt` - format text
- [ ] `fold` - wrap input lines to fit in specified width
- [ ] `head` - output first lines of file(s)
- [ ] `join` - join lines of two files on common fields
- [ ] `md5sum` - compute and check MD5 message digset
- [ ] `nl` - number lines of files
- [ ] `numfmt` - reformat numbers
- [ ] `od` - dump file(s) in octal/other formats
- [ ] `paste` - merge lines of files
- [ ] `ptx` - produces a permuted index of file content
- [ ] `pr` - convert text file(s) for printing
- [ ] `sha1sum`, `sha224sum`, `sha256sum`, `sha384sum`, `sha512sum`- compute & check SHA-1/SHA-2 message digest
- [ ] `shuf` - generate random permutations
- [ ] `sort` - sort lines of text file(s)
- [ ] `split` - split a file into pieces
- [ ] `sum` - checksums and count the blocks in a file
- [ ] `tac` - concatenate and print file(s) in reverse order line by line
- [ ] `tail` - output the last lines of file(s)
- [ ] `tr` - translate or remove characters
- [ ] `tsort` - perform a topological sort
- [ ] `unexpand` - convert spaces to tabs
- [ ] `uniq` - remove duplicate lines from a sorted file
- [ ] `wc` - output bytes/words/lines in file(s)

</details>

---

<details open markdown="1"><summary><h3>Shell Utilities</h3></summary>

- [ ] `arch` - output machine hardware name (uname -m equivalent)
- [ ] `basename` - remove path prefix from pathname
- [ ] `chroot` - change root directory
- [ ] `date` - print/set system date & time
- [ ] `dirname` - strip file suffix from filename
- [ ] `du` - shows disk usage
- [ ] `echo` - output a line of text
- [ ] `env` - output & modify environment variables
- [ ] `expr` - evaluate expressions
- [ ] `factor` - factor numbers
- [ ] `false` - return unsuccess
- [ ] `groups` - output groups user is a member of
- [ ] `hostid` - output numeric identifier for current host
- [ ] `id` - print real/effective UID & GID
- [ ] `link` - create a link to a file
- [ ] `logname` - output user's login name
- [ ] `nice` - modify scheduling priority
- [ ] `nohup` - continue running command after log out
- [ ] `nproc` - query (active) processor count
- [ ] `pathchk` - check whether filenames are valid/portable
- [ ] `pinky` - lightweight version of finger
- [ ] `printenv` - output environment variables
- [ ] `printf` - format & output data
- [ ] `pwd` - print working directory
- [ ] `readlink` - output value of symlink
- [ ] `runcon` - run command with security context
- [ ] `seq` - output sequence of numbers
- [ ] `sleep` - delay execution
- [ ] `stat` - output data about inode
- [ ] `stdbuf` - control buffering for commands that use stdio
- [ ] `stty` - change & print terminal line settings
- [ ] `tee` - send output to multiple files
- [ ] `test` - evaluate conditions/expressions
- [ ] `timeout` - run command with a time limit
- [ ] `true` - return success
- [ ] `tty` - output terminal name
- [ ] `uname` - output system information
- [ ] `unlink` - remove file(s)
- [ ] `uptime` - output system running time
- [ ] `users` -  output usernames currently logged into host
- [ ] `who` - output list of all users logged in
- [ ] `whoami` - output effective userid
- [ ] `yes` - output a string repeatedly
- [ ] ~~`[` - evaluate conditions/expressions (`test` equivalent)~~

</details>
