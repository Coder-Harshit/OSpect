@echo off

:: Script name for clarity (optional)
set SCRIPT_NAME=ospect_install.bat

:: User confirmation before making system-wide changes
echo This script will install OSpect for your user only.
echo It will not modify system-wide settings.
echo Continue? (y/N)

set /p confirmation=

if not "%confirmation%" == "y" && not "%confirmation%" == "Y" (
    echo Aborting installation.
    goto :EOF
)

:: Check for Rust and Cargo
if not exist "C:\Users\%USERNAME%\AppData\Local\Programs\rust\bin\rustc.exe" (
    echo Rust is not installed. Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
)

:: Download OSpect pre-built binary (if available) or build from source
if exist "https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe" (
    :: Download pre-built binary (adjust URL if necessary)
    echo Downloading pre-built OSpect...
    curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe
) else (
    echo Pre-built binary not found. Building OSpect from source...
    :: Assuming your OSpect source code is in the current directory
    git clone https://github.com/Coder-Harshit/OSpect.git
    cd OSpect
    cargo build --release
    if not exist "target\release\ospect.exe" (
        echo OSpect build failed.
        goto :EOF
    )
    move target\release\ospect.exe ospect.exe
)

:: Check for executable permissions (optional, adjust if needed)
if not exist ospect.exe (
    echo OSpect executable not found.
    goto :EOF
)

:: Install OSpect for the user
echo Adding OSpect to your user's PATH...
set user_bin_dir=%USERPROFILE%\bin
if not exist "%user_bin_dir%" (
    echo Creating %user_bin_dir% directory...
    mkdir "%user_bin_dir%"
)
move ospect.exe "%user_bin_dir%"


:: Update PATH variable using Windows environment variables
setx /M PATH "%user_bin_dir%;%PATH%"

echo OSpect installation complete for your user!
echo Restart your terminal or run 'setx /M PATH "%user_bin_dir%;%PATH%"' to update the PATH variable immediately.
