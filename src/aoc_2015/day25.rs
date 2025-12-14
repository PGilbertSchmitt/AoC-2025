/* How I derived the equation to find the number of iterations:
I played with the shape of the iterations by row then by column. Starting at row 1 col 1 (which has value 1),
each successive row has an incrementing value: For row 5, we get 1 (+1) 2 (+2) 4 (+3) 7 (+4) 11..., which has a shape like this:
( ) total: 1
( ) total: 2
( )( ) total: 4
( )( )( ) total: 7
( )( )( )( ) total: 11

This alone can be calculated with the equation (x^2 - x) / 2 + 1

When considering the columnns from that point, we continue incrementing from where we left off but with an
an extra increment in between. Since row=5, the last increment was +4. Now we start incrementing with +6.
For column 4, we get 11 (+6) 17 (+7) 24 (+8) 32
( )( )( )( )( )( ) total: 17
( )( )( )( )( )( )( ) total: 24
( )( )( )( )( )( )( )( ) total: 32

This is just a continuation of the same shape, only without the amount equal to the row, so we add the row and col,
get the nth value of the sequence using the equation for that value, then just subtract the row. */
fn calculate_iterations(row: usize, col: usize) -> usize {
    // We want one less because we don't iterate on the initial value
    nth(row + col) - row - 1
}

// Find the nth number in the sequence 1 2 4 7 11 16...
fn nth(n: usize) -> usize {
    ((n * n) - n) / 2 + 1
}

fn calculate_code(iterations: usize) -> usize {
    let mut value = 20151125;
    for _ in 0..iterations {
        value = value * 252533 % 33554393
    }
    value
}

#[test]
fn part_1() {
    let sample_iterations = calculate_iterations(3, 4);
    assert_eq!(7981243, calculate_code(sample_iterations));

    const ROW: usize = 3010;
    const COL: usize = 3019;
    let iterations = calculate_iterations(ROW, COL);
    assert_eq!(8997277, calculate_code(iterations));
}
