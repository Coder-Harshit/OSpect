@echo off
:: SETLOCAL

:: Check if curl is installed
where curl >nul 2>nul
if %errorlevel% neq 0 (
    echo curl is not installed. Please install curl and run this script again.
    exit /b 1
)

:: Check if link.exe is installed (part of Visual Studio Build Tools)
where link >nul 2>nul
if %errorlevel% neq 0 (
    echo Visual Studio Build Tools are not installed. Please install them from:
    echo https://visualstudio.microsoft.com/visual-cpp-build-tools/
    echo Ensure you select the "Desktop development with C++" workload.
    exit /b 1
)

:: Install Rust and Cargo if not already installed
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    set PATH=%PATH%;%USERPROFILE%\.cargo\bin   
)

:: Clone OSpect repo if it doesn't exist
if not exist OSpect (
    echo Cloning OSpect repository...
    git clone https://github.com/Coder-Harshit/OSpect.git
) else (
    echo OSpect directory already exists. Skipping clone.
)
cd OSpect

:: Build & Install
echo Building OSpect...
call cargo build --release
call cargo install --path .

:: Add OSpect to PATH
set PATH=%PATH%;%CD%\target\release

:: Run OSpect help command
::echo Running OSpect help command...
::ospect --help

echo Installation complete! You can now use the 'ospect' command.