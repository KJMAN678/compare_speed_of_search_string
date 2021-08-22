## Rust 処理速度を比較
https://zenn.dev/megane_otoko/articles/059_rust_for_string

```Rust
// 対象としたテキスト
let sentence :&str = "Although this detail has no connection whatever with the real substance of what we are about to relate, it will not be superfluous, if merely for the sake of exactness in all points, to mention here the various rumors and remarks which had been in circulation about him from the very moment when he arrived in the diocese. True or false, that which is said of men often occupies as important a place in their lives, and above all in their destinies, as that which they do. M. Myriel was the son of a councillor of the Parliament of Aix; hence he belonged to the nobility of the bar. It was said that his father, destining him to be the heir of his own post, had married him at a very early age, eighteen or twenty, in accordance with a custom which is rather widely prevalent in parliamentary families. In spite of this marriage, however, it was said that Charles Myriel created a great deal of talk. He was well formed, though rather short in stature, elegant, graceful, intelligent; the whole of the first portion of his life had been devoted to the world and to gallantry";
```
- テキストは下記サイトから引用 レ・ミゼラブルの冒頭の文章  
https://www.gutenberg.org/files/135/135-h/135-h.htm#link2HCH0001

### パターン 1
```Rust
for i in 0..sentence.len() as usize {
  println!("{}", sentence.chars().nth(i).unwrap());
}
```

### パターン 2
```Rust
for c in sentence.chars() {
  println!("{}", c);
}
```

### 処理時間比較
| パターン 1 | パターン 2 |
| ---- | ---- |
| 0.335秒 | 0.230秒 |

- Rust では Python でいうところのスライスのような記載は推奨されていない模様  
https://rust-lang.github.io/rust-clippy/master/#needless_range_loop

- 時間計測については下記サイト参照  
https://qiita.com/pseudo_foxkeh/items/5d5226e3ffa27631e80d
