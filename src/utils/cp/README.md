# cp

Copy files and directories

```sh
cp [OPTION]... <TARGET>... <DESTINATION>
```

## Description
Copy TARGET to or within DESTINATION

In GNU coreutils `cp` requires a destination argument to be supplied but for consistency with other coreutils such as `ln`, we fallback to current working directory


## Options
`-r`, `--recursive`  ..  copy directories and their contents recursively