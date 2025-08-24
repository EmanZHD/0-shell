
## 📁 File Types Cheat Sheet

| File Type       | Symbol | How to Create                                                                    | Notes                  |
| --------------- | ------ | -------------------------------------------------------------------------------- | ---------------------- |
| Directory       | /      | mkdir mydir                                                                      | Standard folder        |
| Symbolic Link   | @      | ln -s target link                                                                | Points to another file |
| Executable File | *      | echo 'echo Hi' > myexec && chmod +x myexec                                       | Script or binary       |
| Socket          | =/ s   | python3 -c "import socket as s; sock=s.socket(s.AF_UNIX); sock.bind('mysocket')" | IPC endpoint           |
| FIFO            | \|     | mkfifo myfifo                                                                    | Named pipe / queue     |
| Door            | >/D    | Only Solaris: use door_create() + fattach()                                      | IPC (Solaris only)     |
