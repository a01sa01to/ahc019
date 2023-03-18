#!/bin/bash

export RUST_BACKTRACE=1
cargo build
if [ $? -ne 0 ]; then
  echo "Build failed..."
  exit 1
fi

cd tools
echo Running Test 0...
../target/debug/a < in/0.txt 1> out/0.txt 2> err/0.txt
echo Running visualizer...
./target/release/vis in/0.txt out/0.txt &> res/0.txt
cat res/0.txt
for i in {1..100}; do
  echo Running Test $i...
  ../target/debug/a < in/$i.txt 1> out/$i.txt 2> err/$i.txt
  echo Running visualizer...
  ./target/release/vis vis in/$i.txt out/$i.txt &> res/$i.txt
  cat res/$i.txt
done
cd ..
python3 avg.py


# Todo
# 1. Generator Rustの{:04}を変える
# 2. まずはinputを0～100まで作る
# 3. out,res,errディレクトリをつくる
# 4. 上のコードを適切にする
# 5. avg.pyのpathを変える
