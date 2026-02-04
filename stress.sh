#!/usr/bin/env bash

# Configuration
DEST_DIR="../novossite/posts"
COUNT=10000

# Create directory if it doesn't exist
mkdir -p "$DEST_DIR"

echo "Generating $COUNT dummy posts for novos..."

for i in $(seq 1 $COUNT); do
  cat <<EOF > "$DEST_DIR/post-$i.md"
---
title: "Stress Test Post $i"
date: 2026-02-03
tags: ["test", "lorem"]
---

## Section for Post $i

Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 

* Item one
* Item two
* Item three

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
EOF
done

echo "finish!"
