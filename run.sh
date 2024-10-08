#!/bin/sh

if [[ $# -lt 1 ]]; then
    echo "[WARN] Flags not found. Compiling..."
    sleep 2
    cargo build
    cargo run --bin relang main.rel
    exit
fi

case $1 in 
    "-r")
        cargo build
        cargo run --bin relang main.rel
    ;;
    "-g")
        echo "[INFO] Generating rust ast..."
        cargo run --bin util src/asts
    ;;
    "-j")
        echo "[INFO] Generating java ast..."
        javac util/GenerateAst.java
        java util.GenerateAst src/asts
    ;;
    *)
        echo "[INFO] Invalid flag"
    ;;
esac

