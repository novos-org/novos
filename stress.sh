#!/usr/bin/env bash

DEST_DIR="../stress/novos/"
COUNT=10000

# Cleanup and setup
rm -rf "$DEST_DIR"
mkdir -p "$DEST_DIR"

echo "Generating $COUNT high-stress chaotic posts for novos..."

TAGS=("rust" "bsd" "omnios" "performance" "spite" "fast" "minimal" "systems" "tokio" "rayon")
LANGS=("rust" "cpp" "html" "toml" "markdown")

for i in $(seq 1 $COUNT); do
    # Metadata randomization
    DAY=$((1 + $RANDOM % 28))
    MONTH=$((1 + $RANDOM % 12))
    T1=${TAGS[$RANDOM % ${#TAGS[@]}]}
    T2=${TAGS[$RANDOM % ${#TAGS[@]}]}
    BODY_SIZE=$((10 + $RANDOM % 40)) # More meat per file

    {
        echo "---"
        echo "title: \"Chaos Post $i: $(fortune -s 2>/dev/null || echo "The inevitable heat death of the universe")\""
        echo "date: 2026-$(printf "%02d" $MONTH)-$(printf "%02d" $DAY)"
        echo "tags: [\"$T1\", \"$T2\"]"
        echo "author: \"Novos Stress Bot\""
        echo "---"
        echo ""
        echo "# The Industrial Revolution and its consequences for $i"
        
        # Inject a heavy code block to trigger Syntect (THE CPU KILLER)
        L=${LANGS[$RANDOM % ${#LANGS[@]}]}
        echo "### Code Sample in $L"
        echo "\`\`\`$L"
        echo "fn main() {
    let mut total = 0;
    for i in 0..1000 {
        total += i;
        println!(\"Novos is flinging string number: {}\", $i);
    }
}"
        echo "\`\`\`"

        # Generate heavy content
        for j in $(seq 1 $BODY_SIZE); do
            echo "#### Section $j"
            echo "> This is a blockquote for $i.$j. It adds depth to the event-stream parsing."
            echo "Lorem ipsum dolor sit amet, **consectetur adipiscing elit**, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
            
            # Add a list to increase pull-parser events
            echo "* Item A for $j"
            echo "* Item B for $j"
            echo "  * Nested Item for extra stress"
        done
        
    } > "$DEST_DIR/stress-post-$i.md"

    # Progress indicator so you know the script isn't dead
    if (( $i % 1000 == 0 )); then echo "Generated $i/10000..."; fi
done

echo "10,000 chaotic files ready for annihilation."
