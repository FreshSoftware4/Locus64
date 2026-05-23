@echo off
set "ROOT=%~dp0.."
if exist "%ROOT%\target\debug\l64-admin.exe" (
  "%ROOT%\target\debug\l64-admin.exe" %*
) else (
  cargo run -q -p l64-admin -- %*
)
