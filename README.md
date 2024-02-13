<h1 align=center>
  mainframe
</h1>
<h3 align=center>
  A lightweight TUI system monitor
</h3>
<div align=center>

  [![Pipeline](https://github.com/mdLafrance/mainframe/actions/workflows/pipeline.yaml/badge.svg)](https://github.com/mdLafrance/mainframe/actions/workflows/pipeline.yaml)
  [![crates.io](https://img.shields.io/crates/v/mainframe)](https://crates.io/crates/mainframe)

</div>

`mainframe` is a terminal gui app for monitoring system performance, meant to be quicker to use than a traditional system monitoring gui.


## Usage
Calling `mainframe` from the command line will start the app (see screenshot below).  

Press `q` at any time to exit.

<details>
  <summary><b>Screenshot</b></summary>
  
  ![image](https://github.com/mdLafrance/mainframe/assets/25206305/fdff2757-c13c-49f9-b005-2952ccda206e)

</details>

## Installation
Install with cargo:
```bash
cargo install mainframe
mainframe --help
```
This builds and installs the `mainframe` binary.

### Future updates
- [ ] Process monitoring tab
- [ ] AMD gpu support
- [ ] Motherboard stats
- [ ] Logs tab

--- 

> [!WARNING]
> Only Nvidia graphics cards are currently supported
