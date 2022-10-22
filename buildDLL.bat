md "build" 2>NUL
:: cl.exe should be x86/32bit version
cl.exe /D_USRDLL /D_WINDLL src/dll.c Shlwapi.lib User32.lib /Fo:build/ /link /DLL /OUT:build/mhf-iel.dll

