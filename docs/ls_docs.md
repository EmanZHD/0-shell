
## 📁 File Types Cheat Sheet

| File Type       | Symbol | How to Create                                                                    | Notes                  |
| --------------- | ------ | -------------------------------------------------------------------------------- | ---------------------- |
| Directory       | /      | mkdir mydir                                                                      | Standard folder        |
| Symbolic Link   | @      | ln -s target link                                                                | Points to another file |
| Executable File | *      | echo 'echo Hi' > myexec && chmod +x myexec                                       | Script or binary       |
| Socket          | =/ s   | python3 -c "import socket as s; sock=s.socket(s.AF_UNIX); sock.bind('mysocket')" | IPC endpoint           |
| FIFO            | \|     | mkfifo myfifo                                                                    | Named pipe / queue     |
| Door            | >/D    | Only Solaris: use door_create() + fattach()                                      | IPC (Solaris only)     |


## TO DO
- error cases to HAndle

```
ls --F
ls: unrecognized option '--F'
```
```
ls emty_Dir -> print /n
```
delete new line paleaaase => assigned by the QUEEN hasnae elamrani

file permissions, number of links, owner, group, size, last modification time, and the name

fix sort of numeric in /dev dir
issue in file size
issue in format of -l
issue in  ls -F /dev format

/dev/tty → terminal device

/dev/random, /dev/urandom → random number generators

/dev/null → data sink
```
ls -l /dev/null
crw-rw-rw- 1 root root 1, 3 Aug 23 10:02 /dev/null
```


```echo $LS_COLORS```

block devices (b) or character devices (c) being colored yellow due to your LS_COLORS configuration.

b → block device (yellow background normally)

c → character device (cyan normally, but yellow if your theme changed)

p → FIFO (should also show | with ls -F)


The major number and minor number tell the kernel how to access the device.