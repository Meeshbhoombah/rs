use std::error::Error;
use std::fs;

/// Enum to represent directions in the word search
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalLR, // Left-to-Right diagonal
    DiagonalRL, // Right-to-Left diagonal
}

#[derive(Debug)]
struct WordSearch {
    grid: Vec<Vec<char>>,
    bad_char: Vec<usize>, // Precomputed bad character table for the pattern
}

impl WordSearch {
    // Create a new WordSearch instance from input and precompute bad_char table
    pub fn new(input: &str, word: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let bad_char = Self::compute_bad_char_table(word);
        WordSearch { grid, bad_char }
    }

    // Compute the bad character table for Boyer-Moore
    fn compute_bad_char_table(pattern: &str) -> Vec<usize> {
        let m = pattern.len();
        let mut bad_char = vec![m; 256]; // ASCII character range

        for (i, &c) in pattern.as_bytes().iter().enumerate() {
            bad_char[c as usize] = m - 1 - i;
        }

        bad_char
    }

    // Boyer-Moore algorithm to find occurrences of a pattern in a text
    fn boyer_moore(&self, text: &[char], pattern: &[char]) -> usize {
        let mut count = 0;
        let m = pattern.len();
        let n = text.len();

        if m == 0 || m > n {
            return 0;
        }

        let mut s = 0;
        while s <= n - m {
            let mut j = (m - 1) as isize;

            while j >= 0 && pattern[j as usize] == text[s + j as usize] {
                j -= 1;
            }

            if j < 0 {
                count += 1; // Found an occurrence
                s += if s + m < n {
                    self.bad_char[text[s + m] as usize]
                } else {
                    1
                };
            } else {
                s += self.bad_char[text[s + j as usize] as usize].max(1);
            }
        }

        count
    }

    // General function to search in a specific direction
    fn search(&self, word: &str, direction: Direction) -> usize {
        let word_chars: Vec<char> = word.chars().collect();
        let word_reversed: Vec<char> = word.chars().rev().collect();
        let mut count = 0;

        match direction {
            Direction::Horizontal => {
                for row in &self.grid {
                    count += self.boyer_moore(row, &word_chars);
                    count += self.boyer_moore(row, &word_reversed);
                }
            }
            Direction::Vertical => {
                let cols = self.grid[0].len();
                for c in 0..cols {
                    let column: Vec<char> = self.grid.iter().map(|row| row[c]).collect();
                    count += self.boyer_moore(&column, &word_chars);
                    count += self.boyer_moore(&column, &word_reversed);
                }
            }
            Direction::DiagonalLR | Direction::DiagonalRL => {
                let rows = self.grid.len();
                let cols = self.grid[0].len();
                for start in 0..(rows + cols - 1) {
                    let mut diagonal = Vec::new();
                    for i in 0..=start {
                        let j = start - i;
                        if direction == Direction::DiagonalLR && i < rows && j < cols {
                            diagonal.push(self.grid[i][j]);
                        } else if direction == Direction::DiagonalRL && j < rows && i < cols {
                            diagonal.push(self.grid[j][cols - 1 - i]);
                        }
                    }
                    count += self.boyer_moore(&diagonal, &word_chars);
                    count += self.boyer_moore(&diagonal, &word_reversed);
                }
            }
        }

        count
    }

    // Search for all occurrences of a word in all directions
    pub fn find_all(&self, word: &str) -> usize {
        let directions = vec![
            Direction::Horizontal,
            Direction::Vertical,
            Direction::DiagonalLR,
            Direction::DiagonalRL,
        ];

        directions
            .iter()
            .map(|dir| self.search(word, dir.clone()))
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input from a file
    let input = fs::read_to_string("day_four_input.txt")?;
    let word = "XMAS";

    // Create the WordSearch instance
    let word_search = WordSearch::new(&input, word);

    // Find all occurrences of the word
    let count = word_search.find_all(word);
    println!("The word '{}' appears {} times.", word, count);

    Ok(())
}

