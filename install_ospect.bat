@echo off
setlocal enabledelayedexpansion

:: Script name for clarity
set SCRIPT_NAME=ospect_install.bat

:: Check if running in GitHub Actions
if defined GITHUB_ACTIONS (
    set CI_MODE=1
    echo Running in GitHub Actions environment
    set confirmation=y
) else (
    set CI_MODE=0
    :: User confirmation before making changes
    echo This script will install OSpect for your user only.
    echo It will not modify system-wide settings.
    echo Continue? (y/N)
    set /p confirmation=
)

if /I not "%confirmation%" == "y" (
    echo Aborting installation.
    exit /b 1
)

:: Setting up directories
set user_bin_dir=%USERPROFILE%\bin
set config_dir=%USERPROFILE%\AppData\Roaming\ospect

:: Create directories if they don't exist
if not exist "%user_bin_dir%" (
    echo Creating %user_bin_dir% directory...
    mkdir "%user_bin_dir%"
)

if not exist "%config_dir%" (
    echo Creating %config_dir% directory...
    mkdir "%config_dir%"
)

:: Download OSpect pre-built binary or build from source
echo Checking for pre-built OSpect binary...
curl --head --silent --fail https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe >nul 2>&1

if %errorlevel% == 0 (
    echo Downloading pre-built OSpect...
    curl -L -o ospect.exe https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/releases/ospect.exe
    curl -L -o config.toml https://raw.githubusercontent.com/Coder-Harshit/OSpect/main/sample_config.toml
) else (
    echo Pre-built binary not found. Building OSpect from source...
    
    :: Check for Rust and Cargo
    where cargo >nul 2>&1
    if %errorlevel% neq 0 (
        echo Rust is not installed. Installing Rust...
        curl -sSf https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -o rustup-init.exe
        rustup-init.exe -y --no-modify-path
        set PATH=%USERPROFILE%\.cargo\bin;%PATH%
        del rustup-init.exe
    )

    :: Clone and build OSpect
    if not exist OSpect (
        git clone https://github.com/Coder-Harshit/OSpect.git
    )
    cd OSpect
    cargo build --release
    
    if not exist "target\release\ospect.exe" (
        echo OSpect build failed.
        exit /b 1
    )
    
    copy /Y target\release\ospect.exe ..\ospect.exe
    if exist "sample_config.toml" (
        copy /Y sample_config.toml ..\config.toml
    )
    cd ..
)

:: Check for executable
if not exist ospect.exe (
    echo OSpect executable not found.
    exit /b 1
)

:: Install OSpect for the user
echo Installing OSpect to %user_bin_dir%...
copy /Y ospect.exe "%user_bin_dir%\"

if exist config.toml (
    copy /Y config.toml "%config_dir%\"
)

:: Update PATH variable
set "path_updated="
echo "%PATH%" | findstr /C:"%user_bin_dir%" >nul 2>&1
if %errorlevel% neq 0 (
    if %CI_MODE% == 1 (
        :: For GitHub Actions
        echo %user_bin_dir%>> %GITHUB_PATH%
        set "PATH=%user_bin_dir%;%PATH%"
    ) else (
        :: For normal Windows installation
        setx PATH "%user_bin_dir%;%PATH%"
        set "PATH=%user_bin_dir%;%PATH%"
    )
    set "path_updated=1"
)

:: Cleanup
if exist OSpect (
    rmdir /s /q OSpect
)

echo OSpect installation complete!

:: Print information about PATH
if defined path_updated (
    echo Added %user_bin_dir% to your PATH.
    if %CI_MODE% == 0 (
        echo You may need to restart your terminal for the PATH changes to take effect.
    )
)

:: Run ospect to verify installation
echo Running ospect to verify installation...
"%user_bin_dir%\ospect.exe" --version

endlocal