#db first to entity
dbfirst() {
    sea-orm-cli generate entity -o ./src/entities
}

test() {
    # 运行单元测试打印println
    cargo test -- --nocapture
}

build() {
    cargo zigbuild -r --target x86_64-unknown-linux-musl
}

$@
