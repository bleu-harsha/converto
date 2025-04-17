@echo off
REM === Converto: Basic File Converter ===

REM Input args
SET "INPUT=%1"
SET "OUTPUT=%2"
SET "MODE=%3"

IF "%INPUT%"=="" (
    echo Usage: converto input.txt output.txt mode
    exit /b
)

IF NOT EXIST "%INPUT%" (
    echo File "%INPUT%" not found!
    exit /b
)

REM Prepare output file
IF EXIST "%OUTPUT%" DEL "%OUTPUT%"

FOR /F "usebackq delims=" %%L IN ("%INPUT%") DO (
    SET "LINE=%%L"
    CALL :PROCESS_LINE
)

echo Done.
exit /b

:PROCESS_LINE
REM Mode handling
SETLOCAL ENABLEDELAYEDEXPANSION
SET "LINE=!LINE!"
IF /I "%MODE%"=="reverse" (
    SET "REV="
    FOR /L %%i IN (0,1,1000) DO (
        SET "CHAR=!LINE:~%%i,1!"
        IF "!CHAR!"=="" GOTO :ENDREV
        SET "REV=!CHAR!!REV!"
    )
    :ENDREV
    >>"%OUTPUT%" echo(!REV!
) ELSE IF /I "%MODE%"=="upper" (
    CALL ECHO !LINE!|CMD /C "more" >>"%OUTPUT%"
) ELSE IF /I "%MODE%"=="lower" (
    CALL ECHO !LINE!|CMD /C "more" >>"%OUTPUT%"
) ELSE (
    >>"%OUTPUT%" echo(!LINE!
)
ENDLOCAL
GOTO :EOF
