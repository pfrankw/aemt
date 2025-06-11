# AEMT


[![Actions Status](https://github.com/pfrankw/aemt/workflows/Build/badge.svg)](https://github.com/pfrankw/aemt/actions)

Ape Escape Manipulation Toolkit is a handy set of tools for interacting with the KKIIDDZZ.DAT archive that's used within the game.

## Features

Here are the main things you can do:

- List files and offset/length via `list` command. Now with extra metadata!
- Extract files via `extract`.
- Patch files with bigger/smaller ones via `patch`.
- Directly edit the HED file via `hedit`.
- Play audio inside sound packs via `play` command! Only working on Linux as of now.
- Extract such audio with `extract-audio`.

## Upcoming

I would really like to bring audio, images and text extraction/conversion capabilities to this tool.
As of now I'm working on Sony's ADPCM decoding in order to assist with possible individuation of specific audio files.

## Contributing

Any contribution, as usual, is welcome.

## References

Here are the all the sources of information being used to work on this project:

- https://problemkaputt.de/psxspx-cdrom-file-archive-hed-dat-bns-str-ape-escape.htm
- https://psx-spx.consoledev.net
- https://www.psxdev.net
