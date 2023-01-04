cargo run -q --bin=elliec -- compile ./test_dir/main.ei -s -a -q -o byteCodeAsm
cargo run -q --bin=elliec -- compile ./test_dir/main.ei -s -a -q -o byteCode
if [ $? -eq 0 ]
then
  echo "Assembly Successful"
  echo "\n\n"
  echo "Assembled ./test_dir/main.eic and ./test_dir/main.eia"
  exit 0
else
  echo "Assembly failed"
  exit 1
fi