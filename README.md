# docker-rust-templete

コンテナ起動

```
$ docker run --rm -e USER=$USER -it -w /app -v $(pwd):/app rust:1.30.1
```

cargo実行

```
$ rust-app
$ cargo run --release

   Compiling hello-rust v0.1.0 (/app/hello-rust)
    Finished release [optimized] target(s) in 3.47s
     Running `target/release/hello-rust`
Hello, world!
```