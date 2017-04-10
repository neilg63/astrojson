DATE="-b"$1
TIME="-ut"$2

TOPO="-topo"$4","$3","$5
GEOPOS="-geopos"$4","$3","$5

LC="tr '[A-Z]' '[a-z]'"
KEYCLEAN="sed -r s/^([a-z_-]+)\.?\s\(?([a-z_-]+?)\)?/\1_\2:/"
KEYTIDY="sed -r s/_:/:/g"
DEGREETIDY="sed -r s/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/g"
REM_BODIES="sed -r s/^(cupido|hades|zeus|kronos|apollon|admetos|poseidon|isis-transpluto|nxbiru|nibiru|harrington|leverrier_neptune|adams_neptune|lowell_pluto|pickering_pluto|vulcan|proserpina).*?$//g"

OUT=$DATE" "$TIME" -fPLEBS "$TOPO" -pa"
swetest $OUT | $LC | $KEYCLEAN | $KEYTIDY | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/g' | $REM_BODIES | sed -r 's/^([a-z_-]+):/t.\1:/g' | sed -r 's/^t\.geo_long:.*?//' | sed -r 's/^.*?version.*?$//' | sed -r 's/\s+delta\s*t:?\s*([0-9.-]+).*?/,\1/'

OUT=$DATE" "$TIME" -fPLEBS "$GEOPOS 
swetest $OUT | $LC | $KEYCLEAN | $KEYTIDY | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/g' | sed -r 's/^(date_dmy|ut|et|geo_long|epsilon_true|nutation|mean_node|true_node|mean_apogee|osc_apogee|intp_apogee|intp_perigee):.*?$//g' | $REM_BODIES | sed -r 's/^([a-z_-]+):/g.\1:/g'

HOUSE="-house"$4","$3",W"
swetest $DATE $TIME $HOUSE -p | $LC | egrep '^(ascendant|vertex|mc|armc) ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/^([a-z_-]+)/t.\1:/g'

HOUSE="-house"$4","$3",W"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/W-\1: /g'

HOUSE="-house"$4","$3",E"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/E-\1: /g'

#HOUSE="-house"$4","$3",D"
#swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/D-\1: /g'

#HOUSE="-house"$4","$3",CB"
#swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/CB-\1: /g'

#HOUSE="-house"$4","$3",S"
#swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/S-\1: /g'

HOUSE="-house"$4","$3",O"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/O-\1: /g'

HOUSE="-house"$4","$3",P"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/P-\1: /g'

HOUSE="-house"$4","$3",K"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/K-\1: /g'

HOUSE="-house"$4","$3",B"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/B-\1: /g'

HOUSE="-house"$4","$3",C"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/C-\1: /g'

HOUSE="-house"$4","$3",M"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/M-\1: /g'

HOUSE="-house"$4","$3",R"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/R-\1: /g'

HOUSE="-house"$4","$3",T"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/T-\1: /g'

HOUSE="-house"$4","$3",A"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/A-\1: /g'

HOUSE="-house"$4","$3",X"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/X-\1: /g'

HOUSE="-house"$4","$3",G"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/G-\1: /g'

HOUSE="-house"$4","$3",H"
swetest $DATE $TIME $HOUSE -p | grep 'house ' | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/' | sed -r 's/house\s\s?\s?([0-9]+)\s+/H-\1: /g'

OUT=$(swetest $DATE $TIME -ay0 | grep 'Ayanamsa')
echo "ay-0: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay1 | grep 'Ayanamsa')
echo "ay-1: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay2 | grep 'Ayanamsa')
echo "ay-2: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay3 | grep 'Ayanamsa')
echo "ay-3: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay4 | grep 'Ayanamsa')
echo "ay-4: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay5 | grep 'Ayanamsa')
echo "ay-5: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay6 | grep 'Ayanamsa')
echo "ay-6: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay7 | grep 'Ayanamsa')
echo "ay-7: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay8 | grep 'Ayanamsa')
echo "ay-8: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay9 | grep 'Ayanamsa')
echo "ay-9: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay10 | grep 'Ayanamsa')
echo "ay-10: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay15 | grep 'Ayanamsa')
echo "ay-15: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay16 | grep 'Ayanamsa')
echo "ay-16: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay21 | grep 'Ayanamsa')
echo "ay-21: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay22 | grep 'Ayanamsa')
echo "ay-22: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay23 | grep 'Ayanamsa')
echo "ay-23: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay25 | grep 'Ayanamsa')
echo "ay-25: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

OUT=$(swetest $DATE $TIME -ay26 | grep 'Ayanamsa')
echo "ay-26: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

# OUT=$(swetest $DATE $TIME -ay27 | grep 'Ayanamsa')
# echo "ay-27: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

# OUT=$(swetest $DATE $TIME -ay28 | grep 'Ayanamsa')
# echo "ay-28: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

# OUT=$(swetest $DATE $TIME -ay29 | grep 'Ayanamsa')
# echo "ay-29: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

# OUT=$(swetest $DATE $TIME -ay30 | grep 'Ayanamsa')
# echo "ay-30: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'

# OUT=$(swetest $DATE $TIME -ay35 | grep 'Ayanamsa')
# echo "ay-35: " $OUT | sed -e "s/Ayanamsa\s*//g" | sed -r 's/([0-9][^0-9 ])\s?\s?([0-9][^0-9 ])\s?\s?([0-9])/\1\2\3/'
