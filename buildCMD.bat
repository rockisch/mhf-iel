md "build" 2>NUL
:: cl.exe should be x86/32bit version
cl.exe src/cmd.c Shlwapi.lib User32.lib /Fo:build/ /Fe:build/mhf-iel.exe
