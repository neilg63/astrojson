DATE="-b"$1
TIME="-ut"$2
TOPO="-topo"$4,$3,$5
HOUSE="-house"$4","$3",H"
CMD="swetest "$DATE" "$TIME" "$TOPO" "$HOUSE" -p"
echo $CMD
$CMD | grep 'house'