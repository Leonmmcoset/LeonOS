@echo off

REM Diagnostic batch file for LeonOS build issues
TITLE LeonOS Diagnostic

cls
echo ====================================
echo      LEONOS BUILD DIAGNOSTICS
====================================
echo.

REM 1. Check system environment
echo SYSTEM INFORMATION
echo ------------------------------------
echo OS: %OS%
echo PROCESSOR_ARCHITECTURE: %PROCESSOR_ARCHITECTURE%
echo.

REM 2. Check if cargo is available
echo CARGO CHECK
echo ------------------------------------
where cargo >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo Cargo found: %PATH:;= & echo.         & echo %
    cargo --version
) else (
    echo ERROR: Cargo not found in PATH
)
echo.

REM 3. Check project directory structure
echo PROJECT DIRECTORY STRUCTURE
echo ------------------------------------
dir /AD
if exist "mbr" echo ✓ mbr directory exists & dir /A "mbr" >nul 2>nul && echo    (contains files) || echo    (empty)
if exist "loader" echo ✓ loader directory exists & dir /A "loader" >nul 2>nul && echo    (contains files) || echo    (empty)
if exist "loader2" echo ✓ loader2 directory exists & dir /A "loader2" >nul 2>nul && echo    (contains files) || echo    (empty)
if exist "kernel" echo ✓ kernel directory exists & dir /A "kernel" >nul 2>nul && echo    (contains files) || echo    (empty)
echo.

REM 4. Check disk images
echo DISK IMAGES
echo ------------------------------------
if exist "emtpy60M.img" (echo ✓ emtpy60M.img exists) else (echo ✗ emtpy60M.img not found)
if exist "empty80M.img" (echo ✓ empty80M.img exists) else (echo ✗ empty80M.img not found)
echo.

REM 5. Try a simple build test for one component
echo BUILD TEST (one component)
echo ------------------------------------
if exist "mbr" (
    echo Testing build of MBR component...
    pushd "mbr"
    echo Running: cargo build --verbose
    cargo build --verbose
    echo.
    echo Build exit code: %ERRORLEVEL%
    popd
) else (
    echo mbr directory not found, skipping build test
)
echo.

echo ====================================
echo      DIAGNOSTICS COMPLETE
====================================
pause