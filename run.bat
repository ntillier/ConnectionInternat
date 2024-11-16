@echo off
setlocal enabledelayedexpansion

:: Get the current directory
set CURRENT_DIR=%LocalAppData%\ConnectionInternat

title ConnectionInternat

:: Run the binary with arguments
:: "%CURRENT_DIR%\alacritty-portable.exe" --class=connectionInternat --title "Connection Internat" --config-file=NUL --command %CURRENT_DIR%\$EXE_NAME
cmd /C %CURRENT_DIR%\$EXE_NAME