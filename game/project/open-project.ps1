if (Get-Process -Name "Docker Desktop" -ErrorAction SilentlyContinue) {
	# 既にDocker Desktopが起動している場合はスキップ
	Write-Host "Docker Desktop is already running. Skipping startup."
}
else {
	Write-Host "Starting Docker Desktop..."
	Start-Process "C:\Program Files\Docker\Docker\Docker Desktop.exe"
	sleep 5
	Write-Host "Docker Desktop started.`n"
}

Write-Host "Starting Wireshark..."
Start-Process pwsh -ArgumentList "-Command", "Start-Process 'C:\Program Files\Wireshark\Wireshark.exe'" -WindowStyle Hidden
sleep 2
Write-Host "Wireshark started.`n"

Write-Host "Opening VS Code in devcontainer..."
Start-Process pwsh -ArgumentList "-Command", 'devcontainer open .' -WindowStyle Hidden
sleep 1
Write-Host "VS Code opened in devcontainer.`n"

Write-Host "Complete open project."
sleep 1
