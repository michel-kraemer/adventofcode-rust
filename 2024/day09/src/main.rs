use std::fs;

#[derive(Debug, Clone)]
enum Item {
    Space { len: usize },
    File { id: usize, len: usize },
}

fn checksum(disk: &[Item]) -> usize {
    let mut result = 0;
    let mut pos = 0;
    for d in disk {
        match d {
            Item::Space { len } => pos += len,
            Item::File { id, len } => {
                for _ in 0..*len {
                    result += pos * id;
                    pos += 1;
                }
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
        orig_disk.push(Item::File {
            id,
            len: file_len.to_digit(10).unwrap() as usize,
        });
        id += 1;
        if let Some(space_len) = chars.next() {
            orig_disk.push(Item::Space {
                len: space_len.to_digit(10).unwrap() as usize,
            });
        } else {
            break;
        }
    }

    // remove trailing space
    while let Item::Space { .. } = orig_disk[orig_disk.len() - 1] {
        orig_disk.remove(orig_disk.len() - 1);
    }

    // part 1
    let mut disk1 = orig_disk.clone();
    let mut i = 0;
    while i < disk1.len() {
        if let Item::Space { len: mut space_len } = disk1[i] {
            disk1.remove(i);
            while space_len > 0 {
                let dl = disk1.len();
                let Item::File {
                    id,
                    len: ref mut file_len,
                } = disk1[dl - 1]
                else {
                    panic!("Last entry should always be a file");
                };
                if *file_len <= space_len {
                    space_len -= *file_len;
                    let f = disk1.remove(disk1.len() - 1);
                    while let Item::Space { .. } = disk1[disk1.len() - 1] {
                        disk1.remove(disk1.len() - 1);
                    }
                    disk1.insert(i, f);
                    i += 1;
                } else {
                    *file_len -= space_len;
                    disk1.insert(i, Item::File { id, len: space_len });
                    space_len = 0;
                }
            }
        }
        i += 1;
    }

    let total1 = checksum(&disk1);
    println!("{}", total1);

    // part 2
    let mut disk2 = orig_disk.clone();
    let mut j = disk2.len() - 1;
    loop {
        match disk2[j] {
            Item::Space { .. } => {}
            Item::File { id, len: file_len } => {
                for i in 0..j {
                    match disk2[i] {
                        Item::File { .. } => continue,
                        Item::Space { len: space_len } => {
                            if space_len >= file_len {
                                disk2[j] = Item::Space { len: file_len };
                                disk2[i] = Item::File { id, len: file_len };
                                if space_len > file_len {
                                    disk2.insert(
                                        i + 1,
                                        Item::Space {
                                            len: space_len - file_len,
                                        },
                                    );
                                }
                                break;
                            }
                        }
                    }
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
