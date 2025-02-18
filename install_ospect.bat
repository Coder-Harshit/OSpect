@echo off

:: Script name for clarity
set SCRIPT_NAME=ospect_install.bat

:: User confirmation before making system-wide changes
echo This script will install OSpect for your user only.
echo It will not modify system-wide settings.
echo Continue? (y/N)

set /p confirmation=

if /I not "%confirmation%" == "y" (
    echo Aborting installation.
    goto :EOF
)

:: Download OSpect pre-built binary (if available) or build from source
echo Checking for pre-built OSpect binary...
curl --head --silent --fail https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe >nul
if %errorlevel% == 0 (
    echo Downloading pre-built OSpect...
    curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe
    curl -O https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/sample_config.toml
) else (
    echo Pre-built binary not found. Building OSpect from source...
    :: Check for Rust and Cargo
    if not exist "%USERPROFILE%\.cargo\bin\rustc.exe" (
        echo Rust is not installed. Installing Rust...
        curl -sSf https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -o rustup-init.exe
        rustup-init.exe -y
        del rustup-init.exe
    )
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
set config_dir=%USERPROFILE%\Appdata\Roaming\ospect
if not exist "%user_bin_dir%" (
    echo Creating %user_bin_dir% directory...
    mkdir "%user_bin_dir%"
)
move /Y ospect.exe "%user_bin_dir%"
ren sample_config.toml config.toml
if not exist "%config_dir%" (
    echo Creating %config_dir% directory...
    mkdir "%config_dir%"
)
move /Y config.toml "%config_dir%"

@REM :: Update PATH variable using Windows environment variables
@REM for /f "tokens=1,* delims=;" %%a in ("%PATH%") do (
@REM     if "%%a" neq "%user_bin_dir%" (
@REM         setx PATH "%user_bin_dir%;%PATH%"
@REM     )
@REM )

:: Cleanup
cd ..
if exist OSpect (
    rmdir /s /q OSpect
)

echo OSpect installation complete for your user!

:: For GitHub Actions, update the PATH for subsequent steps
if defined GITHUB_PATH (
    echo %user_bin_dir%>> %GITHUB_PATH%
)

ospect.exe
