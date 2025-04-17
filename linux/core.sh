#!/bin/bash
# === Converto: Basic File Converter ===

INPUT="$1"
OUTPUT="$2"
MODE="$3"

if [[ -z "$INPUT" || -z "$OUTPUT" || -z "$MODE" ]]; then
    echo "Usage: converto input.txt output.txt mode"
    exit 1
fi

if [[ ! -f "$INPUT" ]]; then
    echo "File '$INPUT' not found!"
    exit 1
fi

> "$OUTPUT"  # clear/create output

while IFS= read -r line; do
    case "$MODE" in
        reverse)
            echo "$line" | rev >> "$OUTPUT"
            ;;
        upper)
            echo "$line" | tr '[:lower:]' '[:upper:]' >> "$OUTPUT"
            ;;
        lower)
            echo "$line" | tr '[:upper:]' '[:lower:]' >> "$OUTPUT"
            ;;
        *)
            echo "$line" >> "$OUTPUT"
            ;;
    esac
done < "$INPUT"

echo "Done."
