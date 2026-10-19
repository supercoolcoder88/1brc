```
```
 hyperfine --warmup 1 --runs 1 './target/release/brc 1b'
 hyperfine --warmup 1 --runs 1 './target/release/brc 1m'
hyperfine --warmup 1 --runs 1 --export-json results.json './target/release/brc 1b'
```
```
