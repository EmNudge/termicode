# Termicode

Baby's first rust project!

This is a terminal TUI unicode search tool.

<img width="700" alt="image" src="https://user-images.githubusercontent.com/24513691/170420509-b99eca42-4d32-4ad7-9760-1d5d61e5c18d.png">

## Options
- `--copy` 
    - copy to clipboard
- `--output=` one of (`symbol`, `name`, `codepoint`)
    - output only one piece of data to stdout. Useful for combining with other commands.

## Searching
Currently the searchbox filters by name across all non-collapsed unicode characters from [UnicodeData.txt](https://unicode.org/Public/UNIDATA/UnicodeData.txt).
Regex is also accepted by the input. It is parsed as regex if surrounded with `/` (e.g. `/smiling.+eyes/`)
