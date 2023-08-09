# Change to the directory where server.exe is located
cd "server\target\debug"
Start-Process .\server.exe

# Sleep for a few seconds to allow the server to start
Start-Sleep -Seconds 2

# Change to the directory where client.exe is located
cd "client\target\debug"
Start-Process .\client.exe

# Pause the script to keep the console window open
Read-Host -Prompt "Press Enter to exit..."