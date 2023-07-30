cd .\ellie_engine
echo "Releasing the engine"

cd .\core
echo "Releasing the core"
cargo publish

cd ..

cd .\tokenizer

echo "Releasing the tokenizer"
cargo publish

cd ..

cd .\parser

echo "Releasing the parser"
cargo publish

cd ..

cd .\bytecode

echo "Releasing the bytecode"
cargo publish

cd ..

cd .\fmt

echo "Releasing the fmt"
cargo publish

cd..

cd .\vm

echo "Releasing the vm"
cargo publish

cd ..

echo "Release complete"