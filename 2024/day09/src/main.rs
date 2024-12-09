use std::fs;

const SPACE: usize = usize::MAX;

#[derive(Debug, Copy, Clone)]
struct Item {
    id: usize,
    len: usize,
}

fn checksum(disk: &[Item]) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for d in disk {
        if d.id == SPACE {
            pos += d.len;
        } else {
            for _ in 0..d.len {
                result += pos * d.id;
                pos += 1;
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut orig_disk = Vec::new();
    let mut chars = input.trim().chars();
    let mut id = 0;
    while let Some(file_len) = chars.next() {
        orig_disk.push(Item {
            id,
            len: file_len.to_digit(10).unwrap() as usize,
        });
        id += 1;
        if let Some(space_len) = chars.next() {
            orig_disk.push(Item {
                id: SPACE,
                len: space_len.to_digit(10).unwrap() as usize,
            });
        } else {
            break;
        }
    }

    // part 1
    let mut cloned_disk = orig_disk.clone();
    let mut disk1 = Vec::new();
    let mut i = 0;
    let mut j = cloned_disk.len() - 1;
    while i <= j {
        if cloned_disk[i].id != SPACE {
            disk1.push(cloned_disk[i]);
        } else {
            let mut space_len = cloned_disk[i].len;
            while space_len > 0 {
                let mut f = cloned_disk[j];
                while f.id == SPACE {
                    j -= 1;
                    f = cloned_disk[j];
                }

                if f.len <= space_len {
                    space_len -= f.len;
                    j -= 1;
                    disk1.push(f);
                } else {
                    cloned_disk[j].len -= space_len;
                    disk1.push(Item {
                        id: f.id,
                        len: space_len,
                    });
                    space_len = 0;
                }
            }
        }
        i += 1;
    }

    let total1 = checksum(&disk1);
    println!("{}", total1);

    // part 2
    let mut disk2 = orig_disk;
    let mut j = disk2.len() - 1;
    loop {
        if disk2[j].id != SPACE {
            let file_len = disk2[j].len;
            for i in 0..j {
                if disk2[i].id == SPACE && disk2[i].len >= disk2[j].len {
                    let space_len = disk2[i].len;
                    disk2[i] = Item {
                        id: disk2[j].id,
                        len: file_len,
                    };
                    disk2[j].id = SPACE;
                    if space_len > file_len {
                        disk2.insert(
                            i + 1,
                            Item {
                                id: SPACE,
                                len: space_len - file_len,
                            },
                        );
                    }
                    break;
                }
            }
        }
        if j == 0 {
            break;
        }
        j -= 1;
    }

    let total2 = checksum(&disk2);
    println!("{}", total2);
}
