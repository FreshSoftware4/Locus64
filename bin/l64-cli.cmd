@echo off
set "ROOT=%~dp0.."
if exist "%ROOT%\target\debug\l64-cli.exe" (
  "%ROOT%\target\debug\l64-cli.exe" %*
) else (
  cargo run -q -p l64-cli -- %*
)
