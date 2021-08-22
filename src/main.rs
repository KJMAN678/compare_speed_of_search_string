use std::time::Instant;

fn main() {

  // LES MISÉRABLES  from https://www.gutenberg.org/files/135/135-h/135-h.htm#link2HCH0001
  let sentence :&str = "Although this detail has no connection whatever with the real substance of what we are about to relate, it will not be superfluous, if merely for the sake of exactness in all points, to mention here the various rumors and remarks which had been in circulation about him from the very moment when he arrived in the diocese. True or false, that which is said of men often occupies as important a place in their lives, and above all in their destinies, as that which they do. M. Myriel was the son of a councillor of the Parliament of Aix; hence he belonged to the nobility of the bar. It was said that his father, destining him to be the heir of his own post, had married him at a very early age, eighteen or twenty, in accordance with a custom which is rather widely prevalent in parliamentary families. In spite of this marriage, however, it was said that Charles Myriel created a great deal of talk. He was well formed, though rather short in stature, elegant, graceful, intelligent; the whole of the first portion of his life had been devoted to the world and to gallantry";

  // パターン 1
  let start_v1 = Instant::now();

  for i in 0..sentence.len() as usize {
    println!("{}", sentence.chars().nth(i).unwrap());
  }

  let _end_v1 = start_v1.elapsed();

  
  // パターン 2
  let start_v2 = Instant::now();

  for c in sentence.chars() {
    println!("{}", c);
  }

  let _end_v2 = start_v2.elapsed();


  println!("v1 {}.{:03}秒経過しました。", _end_v1.as_secs(), _end_v1.subsec_nanos() / 1_000_000);
  println!("v2 {}.{:03}秒経過しました。", _end_v2.as_secs(), _end_v2.subsec_nanos() / 1_000_000);
}