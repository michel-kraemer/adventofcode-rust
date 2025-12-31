use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let num_elves = input.trim().parse::<usize>().unwrap();

    // Part 1: We consider the game to be round-based. In each round, every
    // second elf is removed from the game. Whenever the number of elves is odd,
    // the last elf also removes the first one. This can be seen in the
    // following two games:
    //
    // Round 1: [1,2,3,4,5,6,7,8,9,10]  # remove every second elf
    // Round 2: [1,3,5,7,9]  # remove every second elf, but also remove the first one
    // Round 3: [5,9]  # remove every second elf
    // Round 4: [5]
    //
    // Round 1: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]  # odd
    // Round 2: [3,5,7,9,11,13,15,17]
    // Round 3: [3,7,11,15]
    // Round 4: [3,11]
    // Round 5: [3]
    //
    // Since we're only interested in the final state where there's one elf
    // left, we just need to track the first elf in the list. As you can see in
    // the games above, in every round, the distance between two elves in the
    // list doubles and the length of the list is halved. Also, whenever the
    // number of elves is odd, we increase the first elf by the current distance
    // times 2.
    let mut front = 1;
    let mut distance = 1;
    let mut len = num_elves;

    while len > 1 {
        if len & 1 != 0 {
            front += distance * 2;
        }
        len >>= 1;
        distance <<= 1;
    }
    println!("{front}");

    // Part 2: The game can be accelerated significantly by using two stacks
    // as follows:
    //
    // ```rust
    // let mut stack1 = VecDeque::from_iter(0..num_elves / 2);
    // let mut stack2 = VecDeque::from_iter(num_elves / 2..num_elves);
    // while !stack1.is_empty() {
    //     stack2.push_back(stack1.pop_front().unwrap());
    //     stack2.pop_front();
    //     if stack1.len() + 1 < stack2.len() {
    //         stack1.push_back(stack2.pop_front().unwrap());
    //     }
    // }
    // println!("{}", stack2.front().unwrap() + 1);
    // ```
    //
    // Let's look at all steps if the number of elves is 16:
    //
    // [ 1, 2, 3, 4, 5, 6, 7, 8]   # stack 1
    // [ 9,10,11,12,13,14,15,16]   # stack 2
    //
    // [ 2, 3, 4, 5, 6, 7, 8]
    // [10,11,12,13,14,15,16, 1]
    //
    // [ 3, 4, 5, 6, 7, 8,11]
    // [12,13,14,15,16, 1, 2]
    //
    // [ 4, 5, 6, 7, 8,11]
    // [13,14,15,16, 1, 2, 3]
    //
    // [ 5, 6, 7, 8,11,14]
    // [15,16, 1, 2, 3, 4]
    //
    // [ 6, 7, 8,11,14]
    // [16, 1, 2, 3, 4, 5]
    //
    // [ 7, 8,11,14, 1]
    // [ 2, 3, 4, 5, 6]
    //
    // [ 8,11,14, 1]
    // [ 3, 4, 5, 6, 7]
    //
    // [11,14, 1, 4]
    // [ 5, 6, 7, 8]
    //
    // [14, 1, 4]
    // [ 6, 7, 8,11]
    //
    // [ 1, 4, 7]
    // [ 8,11,14]
    //
    // [4, 7]
    // [11,14, 1]
    //
    // [ 7,14]
    // [ 1, 4]
    //
    // [14]
    // [ 4, 7]
    //
    // [ 7]
    // [14]
    //
    // []
    // [7]
    //
    // There are a few things to note:
    // 1) The last elf always ends up in the second stack
    // 2) We can find out which elf this is by reading the game from the bottom
    //    to the top. Elf 7 "starts" in stack 2 and stack 1 empty. In the next
    //    step, elf 7 moves to the first stack, and after that to the second
    //    stack again, but this time at the second position. If you read the
    //    game further from the bottom to the top, you can see the following two
    //    patterns:
    //    a) When the elf moves to stack 1, it always starts in the first
    //       position. It then stays in this stack for a certain number of
    //       rounds and moves one step to the right in each round. The length of
    //       both stacks doubles in this time (stack 1 is one element shorter
    //       than stack 2 at the end)
    //    b) The elf then moves to stack 2, where it alternately moves one and
    //       two steps to the right in each round. It stays there for as many
    //       rounds as stack 1 has elements. The length of stack 1 increases by
    //       half of the length of stack 2, and stack 2 has the same number of
    //       elements as stack 1 at the end.
    // 3) These patterns continue until we're at the top of the game. The elf
    //    is then at it's original position 7.
    // 4) Exploiting these patterns, we can make jumps to speed up the game.
    // 5) If it's not possible to jump any more, we can calculate the original
    //    position using simple arithmetic.
    let mut total2 = 0;
    let mut stack_len1 = 1;
    let mut stack_len2 = 1;
    let mut which_stack = 1;
    let mut i = 1;
    while i < num_elves {
        if which_stack == 1 {
            // we're in stack 1
            if i * 2 < num_elves {
                // skip ahead to the round where the elf is in stack 2
                i *= 2;
                stack_len1 = 2 * stack_len1 - 1;
                stack_len2 *= 2;
                which_stack = 2;
            } else {
                // it's not possible to skip ahead anymore, compute the elf's
                // original position
                total2 = num_elves - i;
                break;
            }
        } else if i + stack_len1 < num_elves {
            // we're in stack 2 - skip ahead until the elf is in stack 1 again
            i += stack_len1;
            stack_len1 += stack_len2 / 2;
            stack_len2 = stack_len1;
            which_stack = 1;
        } else {
            // we're in stack 2, but it's not possible to skip ahead anymore
            // compute the original position
            total2 = (num_elves - i) * 2 + stack_len1;
            break;
        }
    }
    println!("{total2}");
}
