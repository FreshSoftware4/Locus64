@echo off
set "ROOT=%~dp0.."
if exist "%ROOT%\target\debug\mf-cli.exe" (
  "%ROOT%\target\debug\mf-cli.exe" %*
) else (
  cargo run -q -p mf-cli -- %*
)
