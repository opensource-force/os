# chmod

change file/directory permissions

```sh
chmod [OPTION]... <MODE> <TARGET>...
#chmod [OPTION]... <MODE[,MODE]> <TARGET>...
```

## Description
Accept octal mode for user/group/other as first argument to modiy the permissions of TARGET. `777`

~~Accept symbolic mode for user/group/other as r (read), w (write), x (execute) and/or - (absent) to modify permissions of TARGET. `rwxrwxrwx`~~

~~Accept symbolic mode for specified u (user), g (group) and/or o (other)/a (all) as r (read), w (write) and/or x (execute) to modify permissions of TARGET. `a=rwx`~~