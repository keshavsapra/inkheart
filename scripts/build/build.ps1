param (
    [string]$buildType
)

# Check if a valid argument has been provided
if (-not ($buildType -eq "docker" -or $buildType -eq "export" -or $buildType -eq "test")) {
    Write-Error "Invalid argument. Please provide 'docker', 'export', or 'test'."
    exit 1
}

$env:DOCKER_BUILDKIT = 1
$env:BUILDKIT_PROGRESS = "plain"

$properties = Get-Content "VERSION" | ForEach-Object { $_ -split '=' } | ForEach-Object { $_.Trim() }
$hashTable = @{}
for ($i = 0; $i -lt $properties.Length; $i += 2) {
    $hashTable[$properties[$i]] = $properties[$i + 1]
}

$version = $hashTable["version"]

if ($buildType -eq "docker") {
    docker build $pwd -f docker/dockerfile -t "nextchapter:${version}" -m 2g
} elseif ($buildType -eq "export") {
    if (-not (Test-Path -Path "./build" -PathType Container)) {
        New-Item -ItemType Directory -Path "./build"
    }
    docker build $pwd -f docker/dockerfile --target export -o build -m 2g
} elseif ($buildType -eq "test") {
    docker build $pwd -f docker/dockerfile -t nextchapter-test --target test -m 2g
    docker run nextchapter-test
} 