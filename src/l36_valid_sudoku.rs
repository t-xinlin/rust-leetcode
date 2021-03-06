#[allow(dead_code)]
/*
36. 有效的数独

判断一个 9x9 的数独是否有效。只需要根据以下规则，验证已经填入的数字是否有效即可。

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。

![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
上图是一个部分填充的有效的数独。

数独部分空格内已填入了数字，空白格用 '.' 表示。

示例 1:

输入:
[
  ["5","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
输出: true
示例 2:

输入:
[
["8","3",".",".","7",".",".",".","."],
["6",".",".","1","9","5",".",".","."],
[".","9","8",".",".",".",".","6","."],
["8",".",".",".","6",".",".",".","3"],
["4",".",".","8",".","3",".",".","1"],
["7",".",".",".","2",".",".",".","6"],
[".","6",".",".",".",".","2","8","."],
[".",".",".","4","1","9",".",".","5"],
[".",".",".",".","8",".",".","7","9"]
]
输出: false
解释: 除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。
     但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
说明:

一个有效的数独（部分已被填充）不一定是可解的。
只需要根据以上规则，验证已经填入的数字是否有效即可。
给定数独序列只包含数字 1-9 和字符 '.' 。
给定数独永远是 9x9 形式的。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/valid-sudoku
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

*/
/*
思路:
看题目觉得需要三遍遍历,答案给出了一遍遍历,初看没看明白
原来是每行,每列都有一个独立的map来存储计数,每个小子宫格也是如此
这样确实是一次遍历,但是存储空间却浪费了更多,
个人感觉还不如逐行逐列判断.
但是如果是在动态的求数独的过程中,显然一遍遍历的方式更合适.
*/
use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = Vec::new();
        let mut cols = Vec::new(); //行列
        let mut boxes = Vec::new();
        for _ in 0..9 {
            rows.push(HashMap::new());
            cols.push(HashMap::new());
            boxes.push(HashMap::new());
        }
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                //println!("test={}",c);
                if c == '.' {
                    continue;
                }
                let bi = (i / 3) * 3 + j / 3;
                if rows[i].contains_key(&c)
                    || cols[j].contains_key(&c)
                    || boxes[bi].contains_key(&c)
                {
                    return false;
                }
                rows[i].insert(c, 1);
                cols[j].insert(c, 1);
                boxes[bi].insert(c, 1);
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
