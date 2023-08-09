# Define project directories
$clientDir = "client"
$serverDir = "server"

# Compile client project
Write-Host "Compiling client project..."
cd $clientDir
cargo build
cd ..

# Compile server project
Write-Host "Compiling server project..."
cd $serverDir
cargo build
cd ..

Write-Host "Compilation completed for both client and server projects."