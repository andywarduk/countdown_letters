# countdown_letters

A Countdown letters game solver written in Rust.

## The Countdown letters game

[Countdown](https://en.wikipedia.org/wiki/Countdown_(game_show)) is a British Channel 4 game show which has been running since November 1982. It was based on the French TV show [Des Chiffres Et Des Lettres](https://en.wikipedia.org/wiki/Des_chiffres_et_des_lettres).

## Game solver

To solve a letters game run the binary with a list of letters:

```sh
cargo run --release -- anedrwips
```

or use the convenience script (unix):

```sh
./solve.sh anedrwips
```

Sample output:

```sh
$ ./solve.sh --min-len 7 anedrwips
49 words found
== 8 letter words (3) ==
  DREPANIS  PREDAWNS  SPRAINED
== 7 letter words (46) ==
  ANDRIES  ASPIRED  DESPAIR  DIAPERS  DIASPER  DIPWARE  DISWARN  ENWRAPS
  EPINARD  INDRAPE  INWARDS  INWRAPS  IPSEAND  ISANDER  PANDERS  PANDIES
  PANIERS  PANSIDE  PANSIED  PARDESI  PARDINE  PAWNERS  PERSIAN  PINARDS
  PINDERS  PRAISED  PRASINE  PRAWNED  PREDAWN  PRESAID  RANDIES  RAPINES
  REWINDS  SANDIER  SAPRINE  SARDINE  SINWARD  SPAWNED  SPAWNER  SPINDER
  SPIRANE  WANDERS  WARDENS  WASPIER  WINDERS  WINESAP
```

## Included word list

The included words.txt file comes from [https://github.com/dwyl/english-words] which originally came from [https://www.infochimps.com/datasets/word-list-350000-simple-english-words-excel-readable].
