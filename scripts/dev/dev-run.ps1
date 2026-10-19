param(
    [switch]$basicauth,
    [switch]$firebase,
    [switch]$basepath,
    [string]$credentials = "user:password"
)

# Validate exclusive switches
$switchCount = 0
if ($basicauth) { $switchCount++ }
if ($firebase) { $switchCount++ }
if ($basepath) { $switchCount++ }

if ($switchCount -gt 1) {
    Write-Host "-basicauth, -firebase, and -basepath are exclusive switches. Please provide only one of them."
    exit
}

# Default to standard .env file if no auth specified
$envFile = "dev/.env"  # Path for --env-file (includes dev/)
$envFileName = ".env"  # For COMPOSE_ENV_FILE (no dev/ prefix)

if ($basicauth) {
    Write-Host "loading basic auth env"
    $username, $password = $credentials -split ":", 2

    # Compute hash over password
    $passwordBytes = [System.Text.Encoding]::ASCII.GetBytes($password)
    $sha1 = [System.Security.Cryptography.SHA1]::Create()
    $hash = $sha1.ComputeHash($passwordBytes)

    $hashedpasswd = [convert]::ToBase64String($hash)

    $htpasswdEntry = "${username}:{SHA}${hashedpasswd}"
    $htpasswdFile = "dev/nginx/.htpasswd"

    $htpasswdEntry | Out-File -FilePath $htpasswdFile -Encoding ASCII

    # use basic auth env
    $envFile = "dev/.env.basicauth"
    $envFileName = ".env.basicauth"
}

if ($firebase) {
    # use firebase env
    Write-Host "loading firebase env"
    $envFile = "dev/.env.firebase"
    $envFileName = ".env.firebase"
    
    # Read the Firebase config file content
    $firebaseConfigPath = "dev/data/config/firebase.json"
    if (Test-Path $firebaseConfigPath) {
        Write-Host "Found Firebase config at $firebaseConfigPath"
        $firebaseConfig = Get-Content -Path $firebaseConfigPath -Raw
        
        # Create a temporary env file by copying the original
        $tempEnvFile = "dev/.env.firebase.temp"
        Copy-Item -Path $envFile -Destination $tempEnvFile
        
        # Properly escape the JSON for build arg
        $escapedConfig = $firebaseConfig.Replace('"', '\"').Replace("`n", "").Replace("`r", "")
        
        # Check if the file ends with a newline, add one if it doesn't
        $content = Get-Content -Path $tempEnvFile -Raw
        if ($content -and -not $content.EndsWith("`n")) {
            Add-Content -Path $tempEnvFile -Value "" -NoNewline
        }
        
        # Add Firebase config to temp env file on a new line
        Add-Content -Path $tempEnvFile -Value "`nFIREBASE_CONFIG=`"$escapedConfig`""
        
        # Use the temp env file
        $envFile = $tempEnvFile
        Write-Host "Using temporary env file with injected Firebase config as build arg"
    } else {
        Write-Host "Warning: Firebase config file not found at $firebaseConfigPath"
        Write-Host "Continuing with standard Firebase env file without config"
    }
}

if ($basepath) {
    Write-Host "loading basepath env - testing subdirectory hosting"
    $envFile = "dev/.env.basepath"
    $envFileName = ".env.basepath"
    Write-Host "App will be available at http://localhost/pdfs"
}

Write-Host "Using env file: $envFile (docker compose) / $envFileName (for container)"

# Set the COMPOSE_ENV_FILE environment variable to just the filename WITHOUT the dev/ prefix
$Env:COMPOSE_ENV_FILE = $envFileName

# Run docker compose with the env file INCLUDING the dev/ prefix
$dockerComposeCommand = "docker compose -f dev/docker-compose.dev.yml --env-file $envFile watch"

Write-Host "running docker: $dockerComposeCommand"
Invoke-Expression $dockerComposeCommand

# Cleanup temp file if created
if ($firebase -and (Test-Path "dev/.env.firebase.temp")) {
    Remove-Item "dev/.env.firebase.temp"
}