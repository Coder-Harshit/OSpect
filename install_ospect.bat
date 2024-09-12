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

:: Clone OSpect repo if it doesn't exist
if exist OSpect (
    echo OSpect directory already exists. Skipping clone.
) else (
    :: Check if git is installed
    where git >nul 2>nul
    if %errorlevel% neq 0 (
        echo git is not installed. Please install git and run this script again.
        goto :eof
    )
    echo Cloning OSpect repository...
    git clone https://github.com/Coder-Harshit/OSpect.git
)
cd OSpect

:: Build & Install
echo Building OSpect...
call cargo build --release
if %errorlevel% neq 0 (
    echo Build failed. Please check the error messages above.
    goto :eof
)

call cargo install --path .

:: Add OSpect to PATH
set "PATH=%PATH%;%CD%\target\release"

echo Installation complete! You can now use the 'ospect' command.
pause

:eof
