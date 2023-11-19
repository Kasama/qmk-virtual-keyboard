QMK Virtual Keyboard
====================

This application can act as a virtual keyboard that uses [QMK](qmk.fm).

It works by using an existing keyboard, then it intercepts allkeycode events and sends them to a connected QMK controller running a version of [this firmware](https://github.com/Kasama/qmk_firmware/blob/master/keyboards/virtual/rp2040/readme.md) which will in turn send the mapped keycode back to the computer.
