# RSLibreCell - a FreeCell implementation

Copyright © 2025 and later: tristhaus

## For Users

The implementation is terminal-based and has a text interface, which is resizable and zoomable in modern terminals. Check the Help window for a listing of active keys.

<div align="center">

![status](/../screenshot/main.png?raw=true)

</div>

You can configure the keys used to actually play the game by placing a file modeled on [config/key_config.json](/config/key_config.json) in the data directory, which is approximately at the following locations:
| Operating System | Approximate Location                                   |
| ---------------- | ------------------------------------------------------ |
| Linux            | `/home/alice/.local/share/rslibrecell`                 |
| macOS            | `/Users/Alice/Library/Application Support/rslibrecell` |
| MS Windows       | `C:\Users\Alice\AppData\Roaming\rslibrecell`           |

The sample file mirrors the default config, which has the home row of a QWERTY US keyboard for the columns, `q`, `w`, `e`, `r` for the cells, and `u`, `i`, `o`, `p` for the foundations.

*Journey Mode* is available using the `!` key, allowing you to play all winnable games of the 64000 available in order. You can also decide to skip a game for now and come back to it later. Your progress is persisted on disk at the following locations:

## For Developers

Currently, I am not looking for contributions to this project.

## License

This project is licensed under the [Gnu Public License v3](./LICENSE).
