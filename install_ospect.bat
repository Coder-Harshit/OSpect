@echo off
@setlocal enableextensions

:: Check if curl is installed
where curl >nul 2>nul
if %errorlevel% neq 0 (
    echo curl is not installed. Please install curl and run this script again.
    goto :eof
)

:: Install Rust and Cargo if not already installed
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    set "PATH=%PATH%;%USERPROFILE%\.cargo\bin"
)

curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect

:: Move ospect to C:\Program Files
move ospect "C:\Program Files"


echo Installation complete! You can now use the 'ospect' command.
pause

:eof
