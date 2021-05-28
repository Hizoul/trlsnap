export DATA_DIR=/datadir
export SRC_DIR=/srcdir
soffice --headless --convert-to csv:"Text - txt - csv (StarCalc)":44,34,76 "papers_transfer_type.ods"
sed -i 's/,/\\/g' "papers_transfer_type.csv"
sed -i 's/@/,/g' "papers_transfer_type.csv"
sed -i "s/'/\"/g" "papers_transfer_type.csv"
cp papers_transfer_type.csv $DATA_DIR
cp papers_transfer_type.csv $SRC_DIR
cd $SRC_DIR
cargo run --release
cp $DATA_DIR/sorted.json $SRC_DIR/web/src/sorted.json
cd $SRC_DIR/web/src
node make_authors_searchable.js