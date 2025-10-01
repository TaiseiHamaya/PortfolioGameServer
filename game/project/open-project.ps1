Write-Host "Starting Docker Desktop..."
Start-Process "C:\Program Files\Docker\Docker\Docker Desktop.exe"
sleep 3
Write-Host "Docker Desktop started.`n"

Write-Host "Opening VS Code in devcontainer..."
Start-Process powershell -ArgumentList 'devcontainer open .' -WindowStyle Hidden
sleep 1
Write-Host "VS Code opened in devcontainer.`n"

Write-Host "Complete open project."
sleep 1
