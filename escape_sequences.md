# Terminal Escape Sequences for Text Editors

## Screen Control
| Description                   | Escape Sequence       |
|-------------------------------|-----------------------|
| Clear entire screen           | `\x1b[2J`             |
| Clear screen from cursor down | `\x1b[0J` or `\x1b[J` |
| Clear screen from cursor up   | `\x1b[1J`             |
| Clear current line            | `\x1b[2K`             |
| Clear line from cursor right  | `\x1b[0K` or `\x1b[K` |
| Clear line from cursor left   | `\x1b[1K`             |

## Cursor Movement
| Description                      | Escape Sequence     |
|----------------------------------|---------------------|
| Move to home (1,1)               | `\x1b[H`            |
| Move to row,col                  | `\x1b[{row};{col}H` |
| Move up n lines                  | `\x1b[{n}A`         |
| Move down n lines                | `\x1b[{n}B`         |
| Move right n columns             | `\x1b[{n}C`         |
| Move left n columns              | `\x1b[{n}D`         |
| Move up 1 line                   | `\x1b[A`            |
| Move down 1 line                 | `\x1b[B`            |
| Move right 1 column              | `\x1b[C`            |
| Move left 1 column               | `\x1b[D`            |
| Move to beginning of next line   | `\x1b[E`            |
| Move to beginning of prev line   | `\x1b[F`            |
| Move to column n in current line | `\x1b[{n}G`         |

## Cursor Visibility & State
| Description             | Escape Sequence |
|-------------------------|-----------------|
| Hide cursor             | `\x1b[?25l`     |
| Show cursor             | `\x1b[?25h`     |
| Save cursor position    | `\x1b[s`        |
| Restore cursor position | `\x1b[u`        |
| Get cursor position     | `\x1b[6n`       |

## Text Colors (Foreground)
| Description | Escape Sequence |
|-------------|-----------------|
| Black       | `\x1b[30m`      |
| Red         | `\x1b[31m`      |
| Green       | `\x1b[32m`      |
| Yellow      | `\x1b[33m`      |
| Blue        | `\x1b[34m`      |
| Magenta     | `\x1b[35m`      |
| Cyan        | `\x1b[36m`      |
| White       | `\x1b[37m`      |
| Default     | `\x1b[39m`      |

## Background Colors
| Description        | Escape Sequence |
|--------------------|-----------------|
| Black background   | `\x1b[40m`      |
| Red background     | `\x1b[41m`      |
| Green background   | `\x1b[42m`      |
| Yellow background  | `\x1b[43m`      |
| Blue background    | `\x1b[44m`      |
| Magenta background | `\x1b[45m`      |
| Cyan background    | `\x1b[46m`      |
