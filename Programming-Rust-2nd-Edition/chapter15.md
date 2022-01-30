# [Rust] �C�e���[�^

## �͂��߂�

�{�e�́A�v���O���~���ORust(��2��)�ƌ����h�L�������g���Q�Ƃ��A�C�e���[�^�Ƃ悭�g�p����ł��낤���\�b�h���l�̕׋��̂��߂ɃA�E�g�v�b�g�������̂ł��B

## �C�e���[�^�Ƃ�

- **�C�e���[�^**�Ƃ́A`std::iter::Iterator`�g���C�g����������C�ӂ̌^�B
- **�C�e���[�g�\**�Ƃ́A`std::iter::IntoIterator`�����������C�ӂ̌^�B
- �C�e���[�^��**�l**�𐶐�����B
- �C�e���[�^����������l��**�A�C�e��**�ƌĂԁB
- ����C�e���[�^����������A�C�e�����󂯎��R�[�h��**�����**�ƌĂԁB

## `iter`���\�b�h��`iter_mut`���\�b�h

�����̃R���N�V�����^���A�s�ώQ�Ƃ�Ԃ�`iter`���\�b�h�ƉώQ�Ƃ�Ԃ�`iter_mut`���\�b�h���������Ă���B

- `iter`

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter();

    assert_eq!(immutable_iter.next(), Some(&1));
    assert_eq!(immutable_iter.next(), Some(&2));
    assert_eq!(immutable_iter.next(), Some(&3));
    assert_eq!(immutable_iter.next(), Some(&4));
    assert_eq!(immutable_iter.next(), None);
}
```

- `iter_mut`

```rust
fn main() {
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    
    assert_eq!(x, &[3, 4, 6]);
}
```

## `IntoIterator`�̎���

����^��`IntoIterator`���������Ă���΁A`into_iter`���\�b�h���Ăяo�����Ƃ��ł���Bfor���[�v�͂����p���Ă���B

�قƂ�ǂ̃R���N�V�����́A������`IntoIterator`���������Ă���B�����̃��\�b�h�́A���ꂼ��s�ώQ��(`&T`)�A�ώQ��(`&mut T`)�A�l���̂���(`T`)���A�C�e���Ƃ��Đ�������C�e���[�^��Ԃ��B

- �s�ώQ�Ƃɑ΂���`into_iter`�́A�A�C�e���ւ̕s�ώQ�Ƃ𐶐�����C�e���[�^��Ԃ�(�܂�A`Item`�^��`&T`�̃C�e���[�^���Ԃ����)�B
- �ώQ�Ƃɑ΂���`into_iter`�́A�A�C�e���ւ̉ώQ�Ƃ𐶐�����C�e���[�^��Ԃ�(�܂�A`Item`�^��`&mut T`�̃C�e���[�^���Ԃ����)
- �R���N�V�����̒l�ɑ΂���`into_iter`�́A�R���N�V�����̏��L�����擾���A�A�C�e����l�ŕԂ��C�e���[�^��Ԃ�(�܂�A`Item`�^��`T`�̃C�e���[�^���Ԃ����)�B�A�C�e���̏��L���͏���҂Ɉڂ�A���̃R���N�V�����͂��̉ߒ��ŏ�����B�C�e���[�^���h���b�v�����ƁA���̃R���N�V�����Ɏc���Ă����v�f�����ׂăh���b�v����A�����k�ƂȂ����R���N�V�������̂Ă���B

## �C�e���[�^�A�_�v�^

�C�e���[�^�́A`IntoIterator`�g���C�g���񋟂��鑽�l�ȃA�_�v�^���\�b�h�𗘗p�ł���B�A�_�v�^�́A1�̃C�e���[�^������A���炩�Ȃ̗L�p�ȓ�����s���āA�ʂ̃C�e���[�^�����B

## [`std::iter::Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

`map`�́A�X�̗v�f�ɑ΂��ăN���[�W����K�p����C�e���[�^�𐶐�����B

```rust
fn main() {
    let text = " ponies \n giraffes\niguanas  \nsquid".to_string();
    
    let v: Vec<&str> = text.lines()
                        .map(|s| s.trim())
                        .collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
}
```

## [`std::iter::Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

`filter`�́A�X�̗v�f�̂����̈ꕔ����菜�����C�e���[�^�𐶐�����B��菜�������́A�N���[�W���Ō��肷��B


```rust
fn main() {
    let text = " ponies \n giraffes\niguanas  \nsquid".to_string();
    
    let v: Vec<&str> = text.lines()
                        .map(|s| s.trim())
                        // `s == "iguanas"`�͂����Ŏ�菜�����
                        .filter(|s| *s != "iguanas")
                        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}
```

## [`std::iter::Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)

`filter`��`map`�̗̂������s���C�e���[�^�𐶐�����B
�Ԃ����C�e���[�^�́A�^����ꂽ�N���[�W����`Some(value)`��Ԃ��l�̂݁B

```rust
use std::str::FromStr;

fn main() {
    let text = "1\nfrond .25  289\n3.1415 estuary\n";

    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) {
            println!("{:4.2}", number.sqrt());
        }
}
```

## [`std::iter::Iterator::flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)

`flat_map`�́A�N���[�W�����Ԃ����C�ӂ̌��̃A�C�e����������������Ԃ��B

```rust
use std::collections::HashMap;

fn main() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["S?o Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}
```

## [`std::iter::Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

`fold`�́A�C�e���[�^������������A�C�e����S�̂ɑ΂���ݐϏ������s���B

```rust
fn total(init: i32, n: i32) -> i32 {
    (0..=n).fold(init, |sum, item| sum + item)
}

fn main() {
    println!("{}", total(0, 3)); // total(0, 3) = 6
}
```

`fold`�́A�A�L�������[�^�ƌĂ΂�鏉���l�Ƃӂ��̈��������N���[�W����^����B
��L�̗�̏ꍇ�Ainit�������l�Ƃ��A`0..=n`�̌X�̒l���󂯎��A�N���[�W��`|sum, item| sum + item`�ɍ��v�l�ƌX�̒l��^���ČĂяo���A�N���[�W���̋A��l���V�������v�l�ƂȂ�܂��B

�����l��0�Ƃ��A0����3�܂ł̍��v�l���v�Z����ꍇ�͈ȉ��̒ʂ�ƂȂ�B

|�v�f|sum|item|result|
|:--:|:--:|:--:|:--:|
||0|||
|0|0|0|0|
|1|0|1|1|
|2|1|2|3|
|3|3|3|6|