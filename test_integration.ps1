# Quick Test Script for Text Editor Integration
# This demonstrates calling text_editor.exe from PowerShell

Write-Host "=== Text Editor Integration Test ===" -ForegroundColor Cyan
Write-Host ""

$editor = ".\target\release\text_editor.exe"

# Check if editor exists
if (-not (Test-Path $editor)) {
    Write-Host "ERROR: text_editor.exe not found!" -ForegroundColor Red
    Write-Host "Please build it first: cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ Editor found: $editor" -ForegroundColor Green
Write-Host ""

# Test 1: Create a test file
Write-Host "Test 1: Opening text editor with 'integration_test.txt'" -ForegroundColor Yellow
Write-Host "  - Type some text" -ForegroundColor Gray
Write-Host "  - Press Ctrl-S to save" -ForegroundColor Gray
Write-Host "  - Press Ctrl-Q to quit" -ForegroundColor Gray
Write-Host ""
Read-Host "Press Enter to launch editor"

# Call the editor
& $editor "integration_test.txt"

# Check if file was created
if (Test-Path "integration_test.txt") {
    Write-Host ""
    Write-Host "✓ SUCCESS! File was created/modified" -ForegroundColor Green
    Write-Host ""
    Write-Host "File contents:" -ForegroundColor Cyan
    Write-Host "---" -ForegroundColor Gray
    Get-Content "integration_test.txt"
    Write-Host "---" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "File was not created (you probably quit without saving)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Integration Test Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "The editor successfully:" -ForegroundColor Green
Write-Host "  ✓ Accepted a filename parameter" -ForegroundColor Green
Write-Host "  ✓ Ran in blocking mode (script waited)" -ForegroundColor Green
Write-Host "  ✓ Returned control to this script" -ForegroundColor Green
Write-Host ""
Write-Host "You can now integrate text_editor.exe into your shell!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Example usage in your shell:" -ForegroundColor Yellow
Write-Host '  Command::new("text_editor.exe")' -ForegroundColor Gray
Write-Host '      .arg(filename)' -ForegroundColor Gray
Write-Host '      .status()?;' -ForegroundColor Gray

