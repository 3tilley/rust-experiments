RUST_BACKTRACE=full cargo -vv +stage1 rustc -- -Z unpretty=hir -C panic=unwind
RUST_BACKTRACE=full cargo -vv +stage1 rustc -- -Z unpretty=hir-tree -C panic=unwind
