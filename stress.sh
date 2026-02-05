#!/usr/bin/env bash

DEST_DIR="../stress/novos"
COUNT=10000

mkdir -p "$DEST_DIR"

echo "Generating $COUNT high-stress dummy posts for novos..."

# Array of random tags to pull from
TAGS=("rust" "bsd" "omnios" "performance" "spite" "fast" "minimal" "systems")

for i in $(seq 1 $COUNT); do
    # Randomize date within the last year
    DAY=$((1 + $RANDOM % 28))
    MONTH=$((1 + $RANDOM % 12))
    
    # Randomize tags (pick 2 random ones)
    T1=${TAGS[$RANDOM % ${#TAGS[@]}]}
    T2=${TAGS[$RANDOM % ${#TAGS[@]}]}
    
    # Randomize body size (between 5 and 50 paragraphs of filler)
    BODY_SIZE=$((5 + $RANDOM % 45))

    {
        echo "---"
        echo "title: \"Stress Test: $(fortune -s 2>/dev/null || echo "Post $i")\""
        echo "date: 2026-$(printf "%02d" $MONTH)-$(printf "%02d" $DAY)"
        echo "tags: [\"$T1\", \"$T2\"]"
        echo "---"
        echo ""
        echo "## Random Header #$i"
        
        # Generate varied body content
        for j in $(seq 1 $BODY_SIZE); do
            echo "This is paragraph $j of a randomized stress test. Novos needs to parse this efficiently."
            echo "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt."
        done
        
    } > "$DEST_DIR/post-$i.md"
done

echo "Done! 10k chaotic files generated."
