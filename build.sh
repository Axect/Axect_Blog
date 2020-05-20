RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps
cd ../Axect_Blog_Gen
sh surge.sh
cd -
