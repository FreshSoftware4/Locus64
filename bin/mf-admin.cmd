@echo off
set "ROOT=%~dp0.."
if exist "%ROOT%\target\debug\mf-admin.exe" (
  "%ROOT%\target\debug\mf-admin.exe" %*
) else (
  cargo run -q -p mf-admin -- %*
)
