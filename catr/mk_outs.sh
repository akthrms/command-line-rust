#! /bin/bash

set -u

ROOT_DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="${ROOT_DIR}/empty.txt"
FOX="${ROOT_DIR}/fox.txt"
SPIDERS="${ROOT_DIR}/spiders.txt"
THE_BUSTLE="${ROOT_DIR}/the_bustle.txt"
ALL="${EMPTY} ${FOX} ${SPIDERS} ${THE_BUSTLE}"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat $FILE > ${OUT_DIR}/${BASENAME}.out
    cat -n $FILE > ${OUT_DIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUT_DIR}/${BASENAME}.b.out
done

cat $ALL > ${OUT_DIR}/all.out
cat -n $ALL > ${OUT_DIR}/all.n.out
cat -b $ALL > ${OUT_DIR}/all.b.out

cat < $THE_BUSTLE > ${OUT_DIR}/$(basename "$THE_BUSTLE").stdin.out
cat -n < $THE_BUSTLE > ${OUT_DIR}/$(basename "$THE_BUSTLE").n.stdin.out
cat -b < $THE_BUSTLE > ${OUT_DIR}/$(basename "$THE_BUSTLE").b.stdin.out
