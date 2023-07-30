cd .\ellie_engine
echo "Releasing the engine"

cd .\core
echo "Releasing the core"
cargo publish -q

cd ..

cd .\renderer_utils
echo "Releasing the renderer_utils"
cargo publish -q

cd ..

cd .\tokenizer

echo "Releasing the tokenizer"
cargo publish -q

cd ..

cd .\parser\standard_rules

echo "Releasing the standard_rules"
cargo publish -q

cd ..

echo "Releasing the parser"
cargo publish -q

cd ..

cd .\bytecode

echo "Releasing the bytecode"
cargo publish -q

cd ..

cd .\fmt

echo "Releasing the fmt"
cargo publish -q

cd..

cd .\vm

echo "Releasing the vm"
cargo publish -q

cd ..

echo "Releasing the engine"
cargo publish -q

echo "Release complete"