# gfg_scan
gfg_scan is a simple rust program that reads from Grab Free Games Steam RSS and send you a notification if there is a new free game.
> Requires [notifu](https://www.paralint.com/projects/notifu/) for sending balloon notifications on windows
```bash
git clone -b master https://github.com/SrTopster/gfg_scan_rs.git
cd gfg_scan_rs
cargo build
cd target/debug
gfg_scan.exe
```
> You can add a shortcut of this executable on your startup folder so it runs on boot <br>
Checkout the linux version [here](https://github.com/SrTopster/gfg_scan_rs/tree/linux).