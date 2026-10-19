#!/bin/bash

# Default values
credentials="user:password"
basicauth=false
firebase=false
basepath=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --basicauth)
            basicauth=true
            shift
            ;;
        --firebase)
            firebase=true
            shift
            ;;
        --basepath)
            basepath=true
            shift
            ;;
        --credentials)
            credentials="$2"
            shift 2
            ;;
        *)
            echo "Unknown parameter: $1"
            exit 1
            ;;
    esac
done

# Validate exclusive switches
switch_count=0
[ "$basicauth" = true ] && ((switch_count++))
[ "$firebase" = true ] && ((switch_count++))
[ "$basepath" = true ] && ((switch_count++))

if [ $switch_count -gt 1 ]; then
    echo "--basicauth, --firebase, and --basepath are exclusive switches. Please provide only one of them."
    exit 1
fi

# Default to standard .env file if no auth specified
env_file="dev/.env"
env_file_name=".env"  # For COMPOSE_ENV_FILE (no dev/ prefix)

if [ "$basicauth" = true ]; then
    echo "Loading basic auth env"
    # Split credentials into username and password
    IFS=':' read -r username password <<< "$credentials"

    # Compute SHA1 hash and convert to base64
    hashed_passwd=$(echo -n "$password" | sha1sum | cut -d' ' -f1 | xxd -r -p | base64)

    # Create htpasswd entry
    htpasswd_entry="${username}:{SHA}${hashed_passwd}"
    htpasswd_file="dev/nginx/.htpasswd"

    # Write to htpasswd file
    echo "$htpasswd_entry" > "$htpasswd_file"

    # Use basic auth env
    env_file="dev/.env.basicauth"
    env_file_name=".env.basicauth"
fi

if [ "$firebase" = true ]; then
    # Use firebase env
    echo "Loading firebase env"
    env_file="dev/.env.firebase"
    env_file_name=".env.firebase"
    
    # Read the Firebase config file content
    firebase_config_path="dev/data/config/firebase.json"
    if [ -f "$firebase_config_path" ]; then
        echo "Found Firebase config at $firebase_config_path"
        # Create a temporary env file by copying the original
        temp_env_file="dev/.env.firebase.temp"
        cp "$env_file" "$temp_env_file"
        
        # Read and escape the Firebase config for build arg
        firebase_config=$(cat "$firebase_config_path" | sed 's/"/\\"/g' | tr -d '\n')
        
        # Check if the file ends with a newline
        if [ -s "$temp_env_file" ] && [ "$(tail -c 1 "$temp_env_file" | xxd -p)" != "0a" ]; then
            # Add a newline if the file doesn't end with one
            echo "" >> "$temp_env_file"
        fi
        
        # Add Firebase config to temp env file on a new line
        echo "FIREBASE_CONFIG=\"$firebase_config\"" >> "$temp_env_file"
        
        # Use the temp env file
        env_file="$temp_env_file"
        echo "Using temporary env file with injected Firebase config as build arg"
    else
        echo "Warning: Firebase config file not found at $firebase_config_path"
        echo "Continuing with standard Firebase env file without config"
    fi
fi

if [ "$basepath" = true ]; then
    echo "Loading basepath env - testing subdirectory hosting"
    env_file="dev/.env.basepath"
    env_file_name=".env.basepath"
    echo "App will be available at http://localhost/pdfs"
fi

echo "Using env file: $env_file (docker compose) / $env_file_name (for container)"

# Export the COMPOSE_ENV_FILE environment variable (without dev/ prefix)
export COMPOSE_ENV_FILE="$env_file_name"

# Run docker compose with the env file (with dev/ prefix)
docker_compose_command="docker compose -f dev/docker-compose.dev.yml --env-file $env_file watch"
echo "Running: $docker_compose_command"
eval "$docker_compose_command"

# Cleanup temp file if created
if [ "$firebase" = true ] && [ -f "dev/.env.firebase.temp" ]; then
    rm "dev/.env.firebase.temp"
fi