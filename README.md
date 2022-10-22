# MHF IELess Launcher

MHF default launcher requires IE to login. IE sucks.

This project reverse engineered the MHF launcher in order to make it possible to boot the game directly, without going through `mhf.exe` and `mhl.dll`.

Besides removing the IE requirement, having a custom launcher gives more freedom on how the server communicates with it.
This makes HTTPS, custom ports, custom formats (JSON) and providing additional information to the launcher possible.

The final launcher also removes the need to modify the launcher (since we're replacing it) **AND** `mhfo-hd.dll` to disable GameGuard.
This is because the call `mhfo-hd.dll` does to run GameGuard checks is actually provided by the launcher, so we can just provide a mock function to it.

## Compile

In order to compile the project, a x86/32bit version of `cl` should be used, since we need to interface with `mhfo-hd.dll`, which was built for x86.

The easiest way to get `cl` is by installing Visual Studio's 'Desktop C++ Dev' package.
If your Visual Studio is from 2019 or before, it should target x86 by default.
Otherwise, you can run the 'x86 Native Tools CMD' shortcut, which will set everything for you.

After having a x86 version of `cl` available on your shell, just run `buildCMD.bat` or `buildDLL.bat`.

## Usage

As of right now, you can either build the projet as a DLL or a CMD application. I'm still not sure how I'll implement a GUI for it, which is why both options are available.

Also as of right now, you'll need to provide by hand the data the server would normally send to the client (charID and logintoken), since I don't feel like parsing the current API format to get that info in a platform I might change in the future.
