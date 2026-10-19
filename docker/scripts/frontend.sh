#!/bin/bash

if [ "$VITE_ENV" = "dev" ]; then
  # Development mode - inject Firebase config and basepath if available
  
  # Inject BASE_PATH
  if [ -n "$BASE_PATH" ]; then
    echo "Injecting BASE_PATH: $BASE_PATH"
    sed -i "s|\$INK:BASEPATH|$BASE_PATH|g" index.html
  else
    echo "No BASE_PATH provided, using default '/'"
    sed -i 's/\$INK:BASEPATH//g' index.html
  fi
  
  # Inject Firebase config if available
  if [ -n "$FIREBASE_CONFIG" ]; then
    echo "Using provided Firebase config from environment variable"
    # Replace auth flag
    sed -i 's/\$INK:USEAUTH/true/g' index.html
    
    # Clean up any escaped quotes
    CLEAN_CONFIG=$(echo "$FIREBASE_CONFIG" | sed 's/\\"/"/g')
    
    # Create a temporary file for the transformation
    tmp_file=$(mktemp)
    
    # Create HTML-safe config by replacing " with &#34; (numeric entity for double quote)
    # Using the numeric entity avoids the sed backreference issue with &quot;
    HTML_SAFE_CONFIG=$(echo "$CLEAN_CONFIG" | sed 's/"/\&#34;/g')
    
    # Replace the placeholder with the HTML-safe config
    # Using perl instead of sed to avoid backreference issues
    perl -p -e "s/\\\$INK:FIREBASE_CONFIG/$HTML_SAFE_CONFIG/g" index.html > "$tmp_file"
    mv "$tmp_file" index.html
    
    echo "Firebase config injected with proper HTML attribute escaping using perl"
  else
    echo "No Firebase config provided, leaving placeholders untouched"
  fi
else
  echo "Building production version"
  npm run build
fi