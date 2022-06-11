#! /bin/bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello" "there" > $OUTDIR/hello2.txt

echo -n "Hello there" > $OUTDIR/hello1_n.txt
echo -n "Hello" "there" > $OUTDIR/hello2_n.txt
