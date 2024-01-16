# About

Simple util that adds the option to send to files to the start menu.

Written in rust as a way to practice some rust, and build something that was bugging me for a while.

Is well known that windows search sucks. Rigth? Then why didnt I just grabbed one of the many well know programs that add improved search to windows?
Cause why do the sensible thing when I can code my own half baked solution. Instead of having to manually add a new program that I want quick acess to the start menu.
I can do it just a right click, and a left click.

## Steps

- Build with cargo (optional)
- Double click exe, (It will create a shortcut of itself, and place it on %appdata%\Microsoft\Windows\SendTo)
- rename as desired (Default is Start Menu);
- The option will be avalible when right click file or folder(folder send to start menu does nothing)

## TODO

- [x] Simplify process
- [x] add icon
- [x] update cargo info
- [x] add util that if runned stand alone, will create a shortcut of self, and send to SendTo
- [x] do nothing if is folder
- [x] switch off strings to paths
- [x] add a sound if shortcut creation is a success or fails
- [ ] add some customization options
- [ ] plan further improvements
