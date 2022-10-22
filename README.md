# MHF IELess Launcher

MHF default launcher requires IE to login. IE sucks.

This project reverse engineered the MHF launcher in order to make it possible to boot the game directly, without going through `mhf.exe` and `mhl.dll`.

If you're wondering 'Why use this instead of the original launcher?', here are some of the issues that are are solved by using a custom launcher:

- Not being locked to IE.
  - Should open a sea of possibilities on how to design the launcher.
  - Won't take 10 seconds for each request.
  - Game might boot under Linux/Steam deck when using Proton/Wine, since IE was the main reason those weren't even options.
- Not being locked to the weird way MHF connects to the server.
  - Allows launcher operations to be implemented using with HTTP(S), JSON, custom ports, etc.
- Not being locked to the operations and data model the original launcher uses.
  - Allows implementing new operations, such as adding separate buttons for 'Sign Up' and 'Login'.
  - Allows storing and displaying extra information. For example, it would be *possible* to get character portraits on the launcher window.
  - Removes the need to modify the launcher (since we're replacing it) and `mhfo-hd.dll` to remove GameGuard, since `mhfo-hd.dll` calls a function provided by the launcher to run GameGuard checks.

It should be noted that currently there's **no GUI available** with this project. The main idea behind it was modeling the data structures in order to be able to directly boot MHF.

## Compile

In order to compile the project, a x86/32bit version of `cl` should be used, since we need to interface with `mhfo-hd.dll`, which was built for x86.

The easiest way to get `cl` is by installing Visual Studio's 'Desktop C++ Dev' package.
If your Visual Studio is from 2019 or before, it should target x86 by default.
Otherwise, you can run the 'x86 Native Tools CMD' shortcut, which will set everything for you.

After having a x86 version of `cl` available on your shell, just run `buildCMD.bat` or `buildDLL.bat`.

## Usage

As of right now, you can either build the projet as a DLL or a CMD application. I'm still not sure how I'll implement a GUI for it, which is why both options are available.

Also as of right now, you'll need to provide by hand the data the server would normally send to the client (charID and logintoken), since I don't feel like parsing the current API format to get that info in a platform I might change in the future.
